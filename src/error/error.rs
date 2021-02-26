#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("an unknown error occurred")]
    Unknown,
}
