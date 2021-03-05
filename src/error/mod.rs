#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty data")]
    EmptyData,

    #[error("empty kind")]
    EmptyKind,

    #[error("empty status")]
    EmptyStatus,

    #[error("invalid input")]
    InvalidInput,

    #[error("sql error: {0}")]
    Sql(#[from] sqlx::Error),

    #[error("an unknown error occurred")]
    Unknown,
}
