use crate::{
    checked_add, checked_sub,
    ud60x18::{div, mul, pow, UNIT},
    FixedPoint,
};
use cosmwasm_std::StdResult;
use ethnum::U256;

// TODO proptest this with decimal256 from cosmwasm-std
impl FixedPoint for U256 {
    fn complement(&self) -> Self {
        UNIT.saturating_sub(*self)
    }
    fn mul(&self, rhs: Self) -> StdResult<Self> {
        mul(*self, rhs)
    }
    fn div(&self, rhs: Self) -> StdResult<Self> {
        div(*self, rhs)
    }
    fn add(&self, rhs: Self) -> StdResult<Self> {
        checked_add(*self, rhs)
    }
    fn sub(&self, rhs: Self) -> StdResult<Self> {
        checked_sub(*self, rhs)
    }
    fn pow_fp(&self, rhs: Self) -> StdResult<Self> {
        pow(*self, rhs)
    }
}
