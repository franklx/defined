#[cfg(feature = "oasgen")]
pub mod oasgen;

#[cfg(feature = "schemars")]
pub mod schemars;

#[cfg(feature = "sea-query")]
pub mod sea;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "specta")]
pub mod specta;

#[cfg(feature = "ts-rs")]
pub mod ts_rs;