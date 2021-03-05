use crate::data::event::CreateEventInput;
use crate::data::{CreateThingInput, Data, Event, Thing};
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

    pub async fn all_events(&self) -> Result<Vec<Event>, Error> {
        let query = "SELECT * FROM events";
        let events: Vec<Event> = sqlx::query_as(query).fetch_all(&self.0).await?;
        Ok(events)
    }

    pub async fn events_by_kind(&self, kind: String) -> Result<Vec<Event>, Error> {
        let query = "SELECT * FROM events WHERE kind = $1";
        let events: Vec<Event> = sqlx::query_as(query).bind(kind).fetch_all(&self.0).await?;
        Ok(events)
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

    pub async fn get_thing_history(&self, id: Uuid) -> Result<Vec<Event>, Error> {
        let query = "SELECT * FROM events WHERE thing_id = $1";
        let events: Vec<Event> = sqlx::query_as(query).bind(id).fetch_all(&self.0).await?;
        Ok(events)
    }

    async fn create_event(&self, input: CreateEventInput) -> Result<Event, Error> {
        if !input.is_valid() {
            return Err(Error::InvalidInput);
        }

        let event = Event::new(input);

        let query = indoc! {r#"
            INSERT INTO events (id, thing_id, kind, timestamp, metadata)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, thing_id, kind, timestamp, metadata
        "#};

        let mut tx = self.0.begin().await?;

        let event_ret: Event = sqlx::query_as(query)
            .bind(event.id)
            .bind(event.thing_id)
            .bind(event.kind)
            .bind(event.timestamp)
            .bind(event.metadata)
            .fetch_one(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(event_ret)
    }

    pub async fn create_thing(&self, input: CreateThingInput) -> Result<Thing, Error> {
        if !input.is_valid() {
            return Err(Error::InvalidInput);
        }

        let thing = Thing::new(input);

        let query = indoc! {r#"
            INSERT INTO things (id, kind, status, metadata, data)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, kind, status, metadata, data
        "#};

        let mut tx = self.0.begin().await?;

        let thing: Thing = sqlx::query_as(query)
            .bind(thing.id)
            .bind(thing.kind)
            .bind(thing.status)
            .bind(thing.metadata)
            .bind(thing.data)
            .fetch_one(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        let _ = self
            .create_event(CreateEventInput::new(thing.id, "create".to_owned(), None))
            .await?;

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

        let _ = self
            .create_event(CreateEventInput::new(
                thing.id,
                "update_kind".to_owned(),
                None,
            ))
            .await?;

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

        let _ = self
            .create_event(CreateEventInput::new(
                thing.id,
                "update_status".to_owned(),
                None,
            ))
            .await?;

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

        let _ = self
            .create_event(CreateEventInput::new(
                thing.id,
                "update_metadata".to_owned(),
                None,
            ))
            .await?;

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

        let _ = self
            .create_event(CreateEventInput::new(
                thing.id,
                "update_data".to_owned(),
                None,
            ))
            .await?;

        Ok(thing)
    }
}
