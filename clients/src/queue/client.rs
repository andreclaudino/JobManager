use std::{collections::HashMap, sync::Arc};

use amiquip::{Connection, Publish, ExchangeDeclareOptions};
use tokio::sync::Mutex;

use crate::entities::publish_response::PublishRequest;

pub struct QueueClient {
    connection: Arc<Mutex<Connection>>,
    exchange_name: String,
}

impl QueueClient {
    pub async fn new(exchange_name: &str, connection_string: &str) -> anyhow::Result<Self> {
        let broker_connection = Connection::insecure_open(connection_string)?;

        let queue_client = Self {
            connection: Arc::new(Mutex::new(broker_connection)),
            exchange_name: exchange_name.to_owned(),
        };

        Ok(queue_client)
    }

    pub async fn publish<I>(&self, task_name: I, task_parameters: &HashMap<String, serde_json::Value>,) -> anyhow::Result<PublishRequest>
    where
        I: Into<String>,
    {
        let routing_key: String = task_name.into();
                                
        let submit_request = PublishRequest::new(task_parameters);
        let content = submit_request.prepare();

        let channel =
            self.connection
                .lock()
                    .await.open_channel(None)?;
        
        let options = ExchangeDeclareOptions{ durable: true, auto_delete: true, ..ExchangeDeclareOptions::default()};
        let exchange =
            channel
                .exchange_declare(amiquip::ExchangeType::Direct, &self.exchange_name, options)?;
        let publish_payload = Publish::new(&content[..], &routing_key);

        exchange.publish(publish_payload)?;
        
        log::info!("Task {task_id} submited to worker {routing_key}", task_id=submit_request.task_id, routing_key=routing_key);

        Ok(submit_request)
    }

}
