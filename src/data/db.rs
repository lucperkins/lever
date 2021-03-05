use crate::data::{Thing, ThingInput};
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

    pub async fn create_thing(&self, thing: ThingInput) -> Result<Thing, Error> {
        let id = Uuid::new_v4();

        let query =
            "INSERT INTO things (id, kind, status, metadata) VALUES ($1, $2, $3, $4) RETURNING id, kind, status, metadata";

        let mut tx = self.0.begin().await?;

        let thing_ret: Thing = sqlx::query_as(query)
            .bind(id)
            .bind(thing.kind)
            .bind(thing.status)
            .bind(thing.metadata)
            .fetch_one(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(thing_ret)
    }
}
