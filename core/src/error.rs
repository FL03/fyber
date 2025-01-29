/*
    Appellation: error <module>
    Contrib: @FL03
*/

#[derive(Debug, thiserror::Error)]
pub enum FyberError {
    #[error("IO error: {0}")]
    StdIO(#[from] std::io::Error),
    #[error("Unknown error: {0}")]
    Unknown(String)
}
