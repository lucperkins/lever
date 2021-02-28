use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

pub type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema() -> GraphQLSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _: &Context<'_>) -> String {
        String::from("hello world")
    }
}
