use crate::data::{Data, Thing, ThingInput};
use crate::error::Error;
use indoc::indoc;
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

    pub async fn create_thing(&self, input: ThingInput) -> Result<Thing, Error> {
        if !input.is_valid() {
            return Err(Error::InvalidInput);
        }

        let id = Uuid::new_v4();

        let query = indoc! {r#"
            INSERT INTO things (id, kind, status, metadata, data)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, kind, status, metadata, data
        "#};

        let mut tx = self.0.begin().await?;

        let thing: Thing = sqlx::query_as(query)
            .bind(id)
            .bind(input.kind)
            .bind(input.status)
            .bind(input.metadata)
            .bind(input.data)
            .fetch_one(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(thing)
    }

    pub async fn update_thing_kind(&self, id: Uuid, kind: String) -> Result<Thing, Error> {
        if kind.is_empty() {
            return Err(Error::EmptyKind);
        }

        let query = indoc! {r#"
            UPDATE things
            SET kind = $1
            WHERE id = $2
            RETURNING id, kind, status, metadata, data
        "#};

        let mut tx = self.0.begin().await?;

        let thing: Thing = sqlx::query_as(query)
            .bind(kind)
            .bind(id)
            .fetch_one(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(thing)
    }

    pub async fn update_thing_status(&self, id: Uuid, status: String) -> Result<Thing, Error> {
        if status.is_empty() {
            return Err(Error::EmptyStatus);
        }

        let query = indoc! {r#"
            UPDATE things
            SET status = $1
            WHERE id = $2
            RETURNING id, kind, status, metadata, data
        "#};

        let mut tx = self.0.begin().await?;

        let thing: Thing = sqlx::query_as(query)
            .bind(status)
            .bind(id)
            .fetch_one(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(thing)
    }

    pub async fn update_thing_metadata(&self, id: Uuid, metadata: Data) -> Result<Thing, Error> {
        if !metadata.is_valid() {
            return Err(Error::EmptyData);
        }

        let query = indoc! {r#"
            UPDATE things
            SET metadata = $1
            WHERE id = $2
            RETURNING id, kind, status, metadata, data
        "#};

        let mut tx = self.0.begin().await?;

        let thing: Thing = sqlx::query_as(query)
            .bind(metadata)
            .bind(id)
            .fetch_one(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(thing)
    }

    pub async fn update_thing_data(&self, id: Uuid, data: Data) -> Result<Thing, Error> {
        if !data.is_valid() {
            return Err(Error::EmptyData);
        }

        let query = indoc! {r#"
            UPDATE things
            SET data = $1
            WHERE id = $2
            RETURNING id, kind, status, metadata, data
        "#};

        let mut tx = self.0.begin().await?;

        let thing: Thing = sqlx::query_as(query)
            .bind(data)
            .bind(id)
            .fetch_one(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(thing)
    }
}
