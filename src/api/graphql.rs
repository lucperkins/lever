use crate::data::{Thing, ThingInput, DB};
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
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_thing(&self, ctx: &Context<'_>, thing: ThingInput) -> FieldResult<Thing> {
        let db = ctx.data::<DB>()?;
        let thing_ret: Thing = db.create_thing(thing).await?;
        Ok(thing_ret)
    }
}
