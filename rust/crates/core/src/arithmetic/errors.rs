use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RingError {
    ZeroModulo,
}

impl fmt::Display for RingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroModulo => f.write_str("ring modulo must be nonzero"),
        }
    }
}

impl Error for RingError {}

#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    ZeroRows,
    UnevenRows { rows: usize, entries: usize },
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroRows => f.write_str("matrix row count must be nonzero"),
            Self::UnevenRows { rows, entries } => write!(
                f,
                "{entries} entries cannot be divided evenly into {rows} rows"
            ),
        }
    }
}

impl Error for MatrixError {}

#[derive(Debug, PartialEq, Eq)]
pub enum LinearSystemError {
    RhsLengthMismatch { rows: usize, rhs_entries: usize },
}

impl fmt::Display for LinearSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RhsLengthMismatch { rows, rhs_entries } => write!(
                f,
                "linear system has {rows} equations but {rhs_entries} right-hand-side entries"
            ),
        }
    }
}

impl Error for LinearSystemError {}
