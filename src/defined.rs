use std::panic;

/// A Custom Option Enum with Undefined [`Defined::Undef`]
/// Works with Juniper and serde
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Defined<T> {
    Undef,
    Def(T),
}

impl<T> Clone for Defined<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Defined::Def(val) => Defined::Def(val.clone()),
            Defined::Undef => Defined::Undef,
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Defined::Def(to), Defined::Def(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

impl<T> Default for Defined<T> {
    fn default() -> Self {
        Self::Undef
    }
}

impl<T> From<T> for Defined<T> {
    fn from(val: T) -> Defined<T> {
        Defined::Def(val)
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

impl<T> Defined<T> {
    /// Returns `true` if the defined is a [`Defined::Def`] value
    ///
    /// # Examples
    ///
    /// ```
    /// use defined::Defined;
    /// let x: Defined<u32> = Defined::Def(1);
    /// assert_eq!(x.is_def(), true);
    ///
    /// let x: Defined<u32> = Defined::Undef;
    /// assert_eq!(x.is_def(), false);
    /// ```
    #[must_use = "if you intended to assert that this has a value, consider `.unwrap()` instead"]
    #[inline]
    pub const fn is_def(&self) -> bool {
        matches!(*self, Defined::Def(_))
    }

    /// Returns `true` if the option is a [`Defined::Def`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use defined::Defined;
    /// let x: Defined<u32> = Defined::Def(2);
    /// assert_eq!(x.is_def_and(|x| x > 1), true);
    ///
    /// let x: Defined<u32> = Defined::Def(0);
    /// assert_eq!(x.is_def_and(|x| x > 1), false);
    ///
    /// let x: Defined<u32> = Defined::Undef;
    /// assert_eq!(x.is_def_and(|x| x > 1), false);
    /// ```
    #[must_use]
    #[inline]
    pub fn is_def_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Defined::Def(x) => f(x),
            _ => false,
        }
    }

    #[inline]
    pub const fn as_ref(&self) -> Defined<&T> {
        match *self {
            Defined::Def(ref val) => Defined::Def(val),
            Defined::Undef => Defined::Undef,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Defined<&mut T> {
        match *self {
            Defined::Def(ref mut val) => Defined::Def(val),
            Defined::Undef => Defined::Undef,
        }
    }

    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Defined::Def(val) => val,
            _ => expect_failed(msg),
        }
    }

    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Defined::Def(val) => val,
            _ => panic!("called Defined::unwrap() on a `Defined::Undef` value"),
        }
    }

    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Defined::Def(val) => val,
            _ => default,
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Defined::Def(val) => val,
            _ => f(),
        }
    }

    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Defined::Def(x) => x,
            _ => T::default(),
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> Defined<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Defined::Def(val) => Defined::Def(f(val)),
            Defined::Undef => Defined::Undef,
        }
    }

    #[inline]
    pub fn inspect<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let Defined::Def(ref val) = self {
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
            Defined::Def(val) => f(val),
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
            Defined::Def(val) => f(val),
            _ => default(),
        }
    }

    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Defined::Def(val) => Ok(val),
            _ => Err(err),
        }
    }

    #[inline]
    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Defined::Def(val) => Ok(val),
            _ => Err(err()),
        }
    }

    /// Returns `default` if the defined is a [`Defined::Undef`] value
    /// Otherwise, Option<Self>
    ///
    /// # Examples
    ///
    /// ```
    /// use defined::Defined;
    /// let x: Defined<u32> = Defined::Def(1);
    /// assert_eq!(x.def_or(3).unwrap(), 1);
    ///
    /// let x: Defined<u32> = Defined::Undef;
    /// assert_eq!(x.def_or(3).unwrap(), 3);
    /// ```
    #[inline]
    pub fn def_or(self, default: T) -> Option<T> {
        match self {
            Defined::Def(val) => Option::Some(val),
            Defined::Undef => Option::Some(default),
        }
    }
    /// Returns `default` if the defined is a [`Defined::Undef`] value
    /// Otherwise, Option<Self>
    ///
    /// # Examples
    ///
    /// ```
    /// use defined::Defined;
    /// let x: Defined<u32> = Defined::Def(1);
    /// assert_eq!(x.map_def_or(Option::None, |x| x).unwrap(), 1);
    ///
    /// let x: Defined<u32> = Defined::Undef;
    /// assert_eq!(x.map_def_or(Option::Some(0), |x| x).unwrap(), 0);
    /// ```
    #[inline]
    pub fn map_def_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(Option<T>) -> U,
    {
        match self {
            Defined::Def(val) => f(Option::Some(val)),
            _ => default,
        }
    }
}
impl<T> From<Defined<T>> for Option<Option<T>> {
    fn from(value: Defined<T>) -> Self {
        match value {
            Defined::Def(val) => Some(Some(val)),
            Defined::Undef => Option::<Option<T>>::Some(None),
        }
    }
}

impl<T> From<Defined<T>> for Option<T> {
    fn from(value: Defined<T>) -> Self {
        match value {
            Defined::Def(val) => Some(val),
            _ => Option::None,
        }
    }
}
