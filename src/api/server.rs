use crate::api::{graphql_schema, GraphQLSchema};
use crate::error::Error;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{graphql_subscription, Response};
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::{http::Response as HttpResponse, Filter};

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn run(&self) -> Result<(), Error> {
        let schema = graphql_schema();

        let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
            |(schema, request): (GraphQLSchema, async_graphql::Request)| async move {
                let schema_res = schema.execute(request).await;
                Ok::<_, Infallible>(Response::from(schema_res))
            },
        );

        let graphql_playground = warp::path::end().and(warp::get()).map(|| {
            let playground_html =
                playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));

            HttpResponse::builder()
                .header("content-type", "text/html")
                .body(playground_html)
        });

        let routes = graphql_subscription(schema.clone())
            .or(graphql_playground)
            .or(graphql_post);

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));

        println!("Starting the server on port {}", self.port);

        let _ = warp::serve(routes).run(addr).await;

        Ok(())
    }
}
