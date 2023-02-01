use arguments::Arguments;
use task_manager::client::TaskManagerClient;
use server::start_server;

mod arguments;
mod server;
mod task_manager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let parameters = Arguments::load();
    let queue_client = TaskManagerClient::new(&parameters.exchange_name, &parameters.broker_uri).await?;
    start_server(queue_client, &parameters.host, &parameters.port).await?;

    Ok(())
}
