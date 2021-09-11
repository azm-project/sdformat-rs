use thiserror::Error;

pub type Result<T> = std::result::Result<T, SdfError>;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct SdfError(#[from] ErrorKind);

#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error(transparent)]
    Xml(#[from] serde_xml_rs::Error),
    #[error(transparent)]
    RustyXml(#[from] xml::BuilderError),
    #[error("command error {}", .0)]
    Command(String),
}

impl SdfError {
    pub(crate) fn new(err: impl Into<ErrorKind>) -> Self {
        Self(err.into())
    }
}

impl From<std::io::Error> for SdfError {
    fn from(err: std::io::Error) -> SdfError {
        ErrorKind::File(err).into()
    }
}

impl From<&str> for SdfError {
    fn from(err: &str) -> SdfError {
        ErrorKind::Command(err.to_owned()).into()
    }
}

impl From<std::string::FromUtf8Error> for SdfError {
    fn from(err: std::string::FromUtf8Error) -> SdfError {
        ErrorKind::Command(err.to_string()).into()
    }
}
