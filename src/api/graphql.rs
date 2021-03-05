use crate::data::{CreateThingInput, Data, Event, Thing, DB};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use uuid::Uuid;

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn schema(db: DB) -> GraphQLSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn things(&self, ctx: &Context<'_>) -> FieldResult<Vec<Thing>> {
        let db = ctx.data::<DB>()?;
        let things: Vec<Thing> = db.all_things().await?;
        Ok(things)
    }

    async fn thing_by_id(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<Option<Thing>> {
        let db = ctx.data::<DB>()?;
        let thing: Option<Thing> = db.get_thing_by_id(id).await?;
        Ok(thing)
    }

    async fn thing_history(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<Vec<Event>> {
        let db = ctx.data::<DB>()?;
        let events: Vec<Event> = db.get_thing_history(id).await?;
        Ok(events)
    }

    async fn events(&self, ctx: &Context<'_>) -> FieldResult<Vec<Event>> {
        let db = ctx.data::<DB>()?;
        let events: Vec<Event> = db.all_events().await?;
        Ok(events)
    }

    async fn events_by_kind(&self, ctx: &Context<'_>, kind: String) -> FieldResult<Vec<Event>> {
        let db = ctx.data::<DB>()?;
        let events: Vec<Event> = db.events_by_kind(kind).await?;
        Ok(events)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_thing(&self, ctx: &Context<'_>, input: CreateThingInput) -> FieldResult<Thing> {
        let db = ctx.data::<DB>()?;
        let thing: Thing = db.create_thing(input).await?;
        Ok(thing)
    }

    async fn update_thing_kind(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        kind: String,
    ) -> FieldResult<Thing> {
        let db = ctx.data::<DB>()?;
        let thing: Thing = db.update_thing_kind(id, kind).await?;
        Ok(thing)
    }

    async fn update_thing_status(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        status: String,
    ) -> FieldResult<Thing> {
        let db = ctx.data::<DB>()?;
        let thing: Thing = db.update_thing_status(id, status).await?;
        Ok(thing)
    }

    async fn update_thing_metadata(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        metadata: Data,
    ) -> FieldResult<Thing> {
        let db = ctx.data::<DB>()?;
        let thing: Thing = db.update_thing_metadata(id, metadata).await?;
        Ok(thing)
    }

    async fn update_thing_data(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        data: Data,
    ) -> FieldResult<Thing> {
        let db = ctx.data::<DB>()?;
        let thing: Thing = db.update_thing_data(id, data).await?;
        Ok(thing)
    }
}
