use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SnapshotError {
    ErrWhenFetchData,
    ErrStatusCodeInvalid,
    ErrWhenParseBodyResultToBytes,
    ErrContentInvalid,
}

impl Error for SnapshotError {
    fn description(&self) -> &str {
        match self {
            SnapshotError::ErrWhenFetchData => "",
            SnapshotError::ErrStatusCodeInvalid => "",
            SnapshotError::ErrWhenParseBodyResultToBytes => "",
            SnapshotError::ErrContentInvalid => "",
        }
    }
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnapshotError::ErrWhenFetchData => write!(f, ""),
            SnapshotError::ErrStatusCodeInvalid => write!(f, ""),
            SnapshotError::ErrWhenParseBodyResultToBytes => write!(f, ""),
            SnapshotError::ErrContentInvalid => write!(f, ""),
        }
    }
}
