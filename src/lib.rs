//! A Custom Option Enum with Undefined
//!
//! `Optional` is a alternative `Option` enum value with [`Optional::Undef`] value.
//! [`Optional::Def`] is defined, and not-null
//! [`Optional::Null`] is defined, but Null
//! [`Optional::Undef`] is undefined
mod optional;
pub use optional::Optional;
pub mod integrations;
