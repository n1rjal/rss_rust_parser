use serde_xml_rs;
use std::fmt;

#[derive(Debug)]
pub enum RssParsingError {
    SerdeError(serde_xml_rs::Error),
    HttpError(reqwest::Error),
}

/// The `impl fmt::Display for RssParsingError` block is implementing the `fmt::Display` trait for the
/// `RssParsingError` enum. This allows for custom formatting of the error message when the
/// `RssParsingError` type is displayed or converted to a string.
impl fmt::Display for RssParsingError {
    /// The `fmt` function formats an error message for different types of RSS parsing errors in Rust.
    ///
    /// Arguments:
    ///
    /// * `f`: `f` is a mutable reference to a `fmt::Formatter` object. This object is used for
    /// formatting and writing output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RssParsingError::HttpError(e) => write!(f, "Http request Error: {:?}", e),
            RssParsingError::SerdeError(e) => write!(f, "Serde parse Error: {:?}", e),
        }
    }
}

/// The `impl From<serde_xml_rs::Error> for RssParsingError` block is implementing the `From` trait for
/// the `serde_xml_rs::Error` type. This allows for converting a `serde_xml_rs::Error` into a
/// `RssParsingError`.
impl From<serde_xml_rs::Error> for RssParsingError {
    fn from(error: serde_xml_rs::Error) -> Self {
        RssParsingError::SerdeError(error)
    }
}

/// The `impl From<reqwest::Error> for RssParsingError` block is implementing the `From` trait for the
/// `reqwest::Error` type. This allows for converting a `reqwest::Error` into a `RssParsingError`.
impl From<reqwest::Error> for RssParsingError {
    fn from(error: reqwest::Error) -> Self {
        RssParsingError::HttpError(error)
    }
}
