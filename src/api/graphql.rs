use crate::data::{Thing, DB};
use async_graphql::{Context, EmptyMutation, EmptySubscription, FieldResult, Object, Schema};
use uuid::Uuid;

pub type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema(db: DB) -> GraphQLSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
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
