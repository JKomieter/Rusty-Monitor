use std::fmt;

use psutil::Error as PsutilError;

#[derive(Debug)]
pub enum MetricsError {
    CPUError,
    MemoryError,
    NetworkError,
    DiskError,
    ProcessError,
    PsutilError(PsutilError),
}

impl fmt::Display for MetricsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MetricsError::CPUError => write!(f, "Error getting CPU metrics"),
            MetricsError::MemoryError => write!(f, "Error getting Memory metrics"),
            MetricsError::NetworkError => write!(f, "Error getting Network metrics"),
            MetricsError::DiskError => write!(f, "Error getting Disk metrics"),
            MetricsError::ProcessError => write!(f, "Error getting Process metrics"),
            MetricsError::PsutilError(ref e) => write!(f, "Psutil error: {}", e),
        }
    }
}

impl From<PsutilError> for MetricsError {
    fn from(error: PsutilError) -> Self {
        MetricsError::PsutilError(error)
    }
}