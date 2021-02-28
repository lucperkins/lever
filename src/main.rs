use lever::api::Server;
use lever::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short = "p", long, default_value = "8080", env = "PORT")]
    port: u16,

    #[structopt(
        short = "d",
        long,
        default_value = "postgres://localhost/lever",
        env = "DATABASE_URL"
    )]
    database_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Opts::from_args();

    let server = Server::new(opts.port, opts.database_url);

    let _ = server.run().await?;

    Ok(())
}
