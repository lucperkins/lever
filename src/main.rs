use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_warp::{graphql_subscription, Response};
use lever::error::Error;
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::{http::Response as HttpResponse, Filter};

type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _: &Context<'_>) -> String {
        String::from("hello world")
    }
}

struct Server {
    port: u16,
    schema: GraphQLSchema,
}

impl Server {
    fn new(port: u16, schema: GraphQLSchema) -> Self {
        Self { port, schema }
    }

    async fn run(&self) -> Result<(), Error> {
        let graphql_post = async_graphql_warp::graphql(self.schema.clone()).and_then(
            |(schema, request): (
                Schema<QueryRoot, EmptyMutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                Ok::<_, Infallible>(Response::from(schema.execute(request).await))
            },
        );

        let graphql_playground = warp::path::end().and(warp::get()).map(|| {
            HttpResponse::builder()
                .header("content-type", "text/html")
                .body(playground_source(
                    GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
                ))
        });

        let routes = graphql_subscription(self.schema.clone())
            .or(graphql_playground)
            .or(graphql_post);

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));

        let _ = warp::serve(routes).run(addr).await;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let server = Server::new(8080, schema);

    let _ = server.run().await?;

    Ok(())
}
