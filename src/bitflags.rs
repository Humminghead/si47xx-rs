//! Re-exports the standard `bitflags!` macro when `defmt` is disabled,
//! or `defmt::bitflags!` when the `defmt` feature is enabled.

#[cfg(not(feature = "defmt"))]
pub use bitflags::bitflags;

#[cfg(feature = "defmt")]
pub use defmt::bitflags;
