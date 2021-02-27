use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_warp::{graphql_subscription, Response};
use lever::error::Error;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _: &Context<'_>) -> String {
        String::from("hello world")
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, request): (
            Schema<QueryRoot, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move { Ok::<_, Infallible>(Response::from(schema.execute(request).await)) },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
            ))
    });

    let routes = graphql_subscription(schema)
        .or(graphql_playground)
        .or(graphql_post);

    let _ = warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;

    Ok(())
}
