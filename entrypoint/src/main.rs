use arguments::Arguments;
use clients::{QueueClient, DatabaseClient};
use server::start_server;

mod arguments;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let parameters = Arguments::load();

    let queue_client =
        QueueClient::new(&parameters.exchange_name, &parameters.broker_uri).await?;

    let database_client =
        DatabaseClient::new(&parameters.database_uri).await?;
        
    start_server(queue_client, database_client, &parameters.host, &parameters.port).await?;

    Ok(())
}
