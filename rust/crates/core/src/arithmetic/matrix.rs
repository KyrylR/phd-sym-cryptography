use super::errors::MatrixError;

pub(crate) struct Matrix<T> {
    rows: usize,
    cols: usize,
    entries: Vec<T>,
}

impl<T> Matrix<T> {
    /// Builds a Matrix from rows count and vector of entries.
    ///
    /// # Errors
    ///
    /// Returns an error when supplied args are 0ed, or provided rows number does not much provided number of entries exactly.
    pub(crate) fn from(rows: usize, entries: Vec<T>) -> Result<Self, MatrixError> {
        let remainder = entries
            .len()
            .checked_rem(rows)
            .ok_or(MatrixError::ZeroRows)?;

        if remainder != 0 {
            return Err(MatrixError::UnevenRows {
                rows,
                entries: entries.len(),
            });
        }

        let cols = entries
            .len()
            .checked_div(rows)
            .ok_or(MatrixError::ZeroRows)?;

        Ok(Self {
            rows,
            cols,
            entries,
        })
    }

    pub(crate) fn rows(&self) -> usize {
        self.rows
    }

    pub(crate) fn cols(&self) -> usize {
        self.cols
    }
}
