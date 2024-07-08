pub enum SmartShredsError {
    IOError(String),
}

impl From<std::io::Error> for SmartShredsError {
    fn from(error: std::io::Error) -> Self {
        SmartShredsError::IOError(format!("{:?}", error))
    }
}

impl std::fmt::Debug for SmartShredsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartShredsError::IOError(error) => write!(f, "IO Error: {}", error),
        }
    }
}