/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! the custom error for events within the protocol

/// A type alias for a [`Result`](core::result::Result) setup with the custom [`Error`] type
pub type Result<T = ()> = core::result::Result<T, crate::Error>;

/// The [`Error`] implementation describes the various errors that can occur within the library
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    CoreError(fyber_core::Error),
}

#[cfg(feature = "alloc")]
#[allow(unreachable_patterns)]
impl From<Error> for fyber_core::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::CoreError(e) => e,
            _ => fyber_core::Error::box_error(err),
        }
    }
}
