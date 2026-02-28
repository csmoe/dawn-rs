use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub(crate) struct DawnError(pub(crate) String);
