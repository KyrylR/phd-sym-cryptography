use super::errors::LinearSystemError;
use super::matrix::Matrix;

pub(crate) struct LinearSystem<T> {
    coeffs: Matrix<T>,
    rhs_vec: Vec<T>,
}

impl<T> LinearSystem<T> {
    /// Builds a Linear System Ax = b, where A is represented by `coeffs` and b represented by the `rhs_vec`
    ///
    /// # Errors
    ///
    /// Returns an error if A and b dimensions are not compatible.
    pub(crate) fn from(coeffs: Matrix<T>, rhs_vec: Vec<T>) -> Result<Self, LinearSystemError> {
        if coeffs.rows() != rhs_vec.len() {
            return Err(LinearSystemError::RhsLengthMismatch {
                rows: coeffs.rows(),
                rhs_entries: rhs_vec.len(),
            });
        }

        Ok(Self { coeffs, rhs_vec })
    }
}
