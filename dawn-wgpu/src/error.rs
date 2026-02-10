use std::fmt;

#[derive(Debug)]
pub(crate) struct DawnError(pub(crate) String);

impl fmt::Display for DawnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DawnError {}
