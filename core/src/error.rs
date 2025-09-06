/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! the core, custom error for the protocol

/// A type alias for a [`Result`](core::result::Result) setup with the custom [`Error`] type
pub type Result<T = ()> = core::result::Result<T, crate::Error>;

/// The [`Error`] implementation describes the various errors that can occur within the library
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    FmtError(core::fmt::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(alloc::boxed::Box<dyn core::error::Error + Send + Sync + 'static>),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(std::io::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "alloc")]
    #[error("[Unknown Error] {0}")]
    Unknown(alloc::string::String),
}

#[cfg(feature = "alloc")]
#[cfg(feature = "alloc")]
#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::Error;
    use alloc::boxed::Box;
    use alloc::string::String;

    impl Error {
        /// a functional constructor for creating a [`BoxError`](Error::BoxError) variant
        pub fn box_error<E>(err: E) -> Self
        where
            E: core::error::Error + Send + Sync + 'static,
        {
            Error::BoxError(Box::new(err))
        }
    }

    impl From<&str> for Error {
        fn from(err: &str) -> Self {
            Error::Unknown(String::from(err))
        }
    }

    impl From<String> for Error {
        fn from(err: String) -> Self {
            Error::Unknown(err)
        }
    }
}
