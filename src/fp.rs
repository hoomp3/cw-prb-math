use cosmwasm_std::StdResult;

pub trait FixedPoint: Sized + Copy {
    /// Returns the complement of a value (1 - x), capped to 0 if x is larger than 1.
    ///
    /// Useful when computing the complement for values with some level of relative error, as it strips this error and
    /// prevents intermediate negative values.
    fn complement(&self) -> Self;
    /// Fixed point multiplication.
    fn mul(&self, rhs: Self) -> StdResult<Self>;
    /// Fixed point division.
    fn div(&self, rhs: Self) -> StdResult<Self>;
    /// Fixed point addition.
    fn add(&self, rhs: Self) -> StdResult<Self>;
    /// Fixed point subtraction.
    fn sub(&self, rhs: Self) -> StdResult<Self>;
    /// Fixed point exponentiation.
    fn pow_fp(&self, rhs: Self) -> StdResult<Self>;
}
