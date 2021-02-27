use crate::error::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct DB(Pool<Postgres>);

impl DB {
    async fn new(conn_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(conn_url)
            .await?;

        Ok(Self(pool))
    }
}
