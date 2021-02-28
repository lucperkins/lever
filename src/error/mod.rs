#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sql error")]
    Sql(#[from] sqlx::Error),

    #[error("an unknown error occurred")]
    Unknown,
}
