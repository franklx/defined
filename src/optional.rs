use std::default;

use diesel::AsExpression;

use std::iter::{self, FromIterator, FusedIterator};
use std::panic;
use std::{
    cmp, convert, hint, mem,
    ops::{self, ControlFlow, Deref, DerefMut},
    slice,
};

// AsExpression
// #[cfg_attr(feature = "diesel", derive(AsExpression))]
// https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs
// https://github.com/diesel-rs/diesel/pull/429
// https://github.com/diesel-rs/diesel/issues/562
// https://docs.rs/diesel/latest/src/diesel/expression/mod.rs.html#289

/// A Custom Option Enum with Undefined [`Optional::Undef`]
/// Works with Juniper and serde
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Optional<T> {
    Undef,
    Null,
    Def(T),
}

impl<T> Clone for Optional<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Optional::Def(val) => Optional::Def(val.clone()),
            Optional::Null => Optional::Null,
            Optional::Undef => Optional::Undef,
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Optional::Def(to), Optional::Def(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

impl<T> Default for Optional<T> {
    fn default() -> Self {
        Self::Undef
    }
}

impl<T> From<T> for Optional<T> {
    fn from(val: T) -> Optional<T> {
        Optional::Def(val)
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(val: Option<T>) -> Optional<T> {
        match val {
            Some(v) => Optional::Def(v),
            None => Optional::Null,
        }
    }
}

// This is a similar function to reduce code size of .expect() and
// produce panic message like std::Option .expect() function
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never))]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[cold]
#[track_caller]
const fn expect_failed(msg: &str) -> ! {
    panic!("{}", msg)
}

impl<T> Optional<T> {
    /// Returns `true` if the optional is a [`Optional::Def`] value
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Optional<u32> = Optional::Def(1);
    /// assert_eq!(x.is_def(), true);
    ///
    /// let x: Optional<u32> = Optional::Undef;
    /// assert_eq!(x.is_def(), false);
    ///
    /// let x: Optional<u32> = Optional::Null;
    /// assert_eq!(x.is_def(), false);
    /// ```
    #[must_use = "if you intended to assert that this has a value, consider `.unwrap()` instead"]
    #[inline]
    pub const fn is_def(&self) -> bool {
        matches!(*self, Optional::Def(_))
    }

    /// Returns `true` if the option is a [`Optional::Def`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Optional<u32> = Optional::Def(2);
    /// assert_eq!(x.is_def_and(|x| x > 1), true);
    ///
    /// let x: Optional<u32> = Optional::Def(0);
    /// assert_eq!(x.is_def_and(|x| x > 1), false);
    ///
    /// let x: Optional<u32> = Optional::Undef;
    /// assert_eq!(x.is_def_and(|x| x > 1), false);
    ///
    /// let x: Optional<u32> = Optional::Null;
    /// assert_eq!(x.is_def_and(|x| x > 1), false);
    /// ```
    #[must_use]
    #[inline]
    pub fn is_def_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Optional::Def(x) => f(x),
            _ => false,
        }
    }

    /// Returns `true` if the optional is a [`Optional::Null`] value
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Optional<u32> = Optional::Def(1);
    /// assert_eq!(x.is_def(), false);
    ///
    /// let x: Optional<u32> = Optional::Undef;
    /// assert_eq!(x.is_def(), false);
    ///
    /// let x: Optional<u32> = Optional::Null;
    /// assert_eq!(x.is_def(), true);
    /// ```
    #[must_use = "if you intended to assert that this doesn't have a value, consider \
                  `.and_then(|_| panic!(\"`Optional` had a value when expected `Optional::Null`\"))` instead"]
    #[inline]
    pub const fn is_null(&self) -> bool {
        matches!(*self, Optional::Null)
    }

    #[inline]
    pub const fn as_ref(&self) -> Optional<&T> {
        match *self {
            Optional::Def(ref val) => Optional::Def(val),
            Optional::Null => Optional::Null,
            Optional::Undef => Optional::Undef,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Optional<&mut T> {
        match *self {
            Optional::Def(ref mut val) => Optional::Def(val),
            Optional::Null => Optional::Null,
            Optional::Undef => Optional::Undef,
        }
    }

    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Optional::Def(val) => val,
            _ => expect_failed(msg),
        }
    }

    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Optional::Def(val) => val,
            _ => panic!("called Optional::unwrap() on a `Optional::Null` value"),
        }
    }

    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Optional::Def(val) => val,
            _ => default,
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Optional::Def(val) => val,
            _ => f(),
        }
    }

    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Optional::Def(x) => x,
            _ => T::default(),
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> Optional<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Optional::Def(val) => Optional::Def(f(val)),
            Optional::Null => Optional::Null,
            Optional::Undef => Optional::Undef,
        }
    }

    #[inline]
    pub fn inspect<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let Optional::Def(ref val) = self {
            f(val);
        }
        self
    }

    #[inline]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Optional::Def(val) => f(val),
            _ => default,
        }
    }

    #[inline]
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        match self {
            Optional::Def(val) => f(val),
            _ => default(),
        }
    }

    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Optional::Def(val) => Ok(val),
            _ => Err(err),
        }
    }

    #[inline]
    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Optional::Def(val) => Ok(val),
            _ => Err(err()),
        }
    }

    /// Returns `default` if the optional is a [`Optional::Undef`] value
    /// Otherwise, Option<Self>
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Optional<u32> = Optional::Def(1);
    /// assert_eq!(x.def_or(3).unwrap(), 1);
    ///
    /// let x: Optional<u32> = Optional::Undef;
    /// assert_eq!(x.def_or(3).unwrap(), 3);
    ///
    /// let x: Optional<u32> = Optional::Null;
    /// assert_eq!(x.def_or(3), Option::None);
    /// ```
    #[inline]
    pub fn def_or(self, default: T) -> Option<T> {
        match self {
            Optional::Def(val) => Option::Some(val),
            Optional::Null => Option::None,
            Optional::Undef => Option::Some(default),
        }
    }
}
impl<T> From<Optional<T>> for Option<Option<T>> {
    fn from(value: Optional<T>) -> Self {
        match value {
            Optional::Def(val) => Some(Some(val)),
            Optional::Null => Option::None,
            Optional::Undef => Option::<Option<T>>::Some(None),
        }
    }
}

impl<T> From<Optional<T>> for Option<T> {
    fn from(value: Optional<T>) -> Self {
        match value {
            Optional::Def(val) => Some(val),
            _ => Option::None,
        }
    }
}
