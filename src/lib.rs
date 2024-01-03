//! A Custom Option Enum with Undefined
//!
//! `Defined` is a alternative `Option` enum value with [`Defined::Undef`] value.
//! [`Defined::Def`] is defined
//! [`Defined::Undef`] is undefined
mod defined;
pub use defined::{Defined::{self, Def, Undef}};
pub mod integrations;
