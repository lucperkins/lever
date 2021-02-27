use crate::data::Thing;
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

pub struct Things(DB);

impl Things {
    pub async fn new(conn_url: &str) -> Result<Self, Error> {
        let db = DB::new(conn_url).await?;

        Ok(Self(db))
    }

    pub async fn all_things(&self) -> Result<Vec<Thing>, Error> {
        let query = "SELECT * FROM things";
        let things: Vec<Thing> = sqlx::query_as(query).fetch_all(&self.0 .0).await?;
        Ok(things)
    }
}
