use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_warp::{graphql_subscription, Response};
use lever::error::Error;
use std::convert::Infallible;
use std::net::SocketAddr;
use structopt::StructOpt;
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
        let schema = self.schema.clone();

        let graphql_post = async_graphql_warp::graphql(schema).and_then(
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

        let routes = graphql_subscription(self.schema.clone())
            .or(graphql_playground)
            .or(graphql_post);

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));

        println!("Starting the server on port {}", self.port);

        let _ = warp::serve(routes).run(addr).await;

        Ok(())
    }
}

#[derive(StructOpt)]
struct Opts {
    #[structopt(short = "p", long, default_value = "8080", env = "PORT")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Opts::from_args();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let server = Server::new(opts.port, schema);

    let _ = server.run().await?;

    Ok(())
}
