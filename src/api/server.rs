use crate::api::{graphql_schema, GraphQLSchema};
use crate::data::DB;
use crate::error::Error;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{graphql_subscription, Response};
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::{http::Response as HttpResponse, Filter};

pub struct Server {
    port: u16,
    database_url: String,
}

impl Server {
    pub fn new(port: u16, database_url: String) -> Self {
        Self { port, database_url }
    }

    pub async fn run(&self) -> Result<(), Error> {
        let db = DB::new(&self.database_url).await?;

        let schema = graphql_schema(db);

        let graphql_post =
            async_graphql_warp::graphql(schema.clone()).and_then(Self::graphql_handle);

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

    async fn graphql_handle(
        (schema, req): (GraphQLSchema, async_graphql::Request),
    ) -> Result<Response, Infallible> {
        let schema_res = schema.execute(req).await;
        let res = Response::from(schema_res);
        Ok(res)
    }
}
