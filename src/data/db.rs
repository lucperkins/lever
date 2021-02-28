use crate::data::Thing;
use crate::error::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct DB(Pool<Postgres>);

impl DB {
    pub async fn new(conn_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(conn_url)
            .await?;

        Ok(Self(pool))
    }

    pub async fn all_things(&self) -> Result<Vec<Thing>, Error> {
        let query = "SELECT * FROM things";
        let things: Vec<Thing> = sqlx::query_as(query).fetch_all(&self.0).await?;
        Ok(things)
    }

    pub async fn get_thing_by_id(&self, id: Uuid) -> Result<Option<Thing>, Error> {
        let query = "SELECT * FROM things WHERE id = $1";
        let thing: Option<Thing> = sqlx::query_as(query)
            .bind(id)
            .fetch_optional(&self.0)
            .await?;
        Ok(thing)
    }
}
