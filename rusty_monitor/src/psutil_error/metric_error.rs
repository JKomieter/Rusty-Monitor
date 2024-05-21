use std::fmt;
use psutil::Error as PsutilError;

#[derive(Debug)]
pub struct MetricsError(pub PsutilError);

impl fmt::Display for MetricsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl From<PsutilError> for MetricsError {
    fn from(error: PsutilError) -> Self {
        MetricsError(error)
    }
}