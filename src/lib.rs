//! Math library for advanced fixed-point math that works with numbers which are considered to have 18 trailing decimals.
//! Uses U256 and I256 from ethnum since cosmwasm-std numbers don't offer the same functionality as U256 and I256.

#[cfg(feature = "ethnum")]
pub use ethnum::*;
pub use common::*;
pub use asm::Asm;

pub mod sd59x18;
pub mod ud60x18;

mod common;
mod asm;
pub(crate) mod tens;

#[cfg(test)]
mod tests;

pub(crate) const UNIT_U128: u128 = 1_000_000_000_000_000_000u128;
pub(crate) const HALF_UNIT_U128: u128 = 500_000_000_000_000_000u128;
pub(crate) const LOG2_E_U128: u128 = 1_442_695_040_888_963_407u128;
//pub(crate) const E_U128: u128 = 2_718_281_828_459_045_235u128;