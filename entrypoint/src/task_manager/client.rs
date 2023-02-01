use amqprs::{connection::{Connection, OpenConnectionArguments}, channel::BasicPublishArguments, BasicProperties};
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::server::entities::get_task_parameters::GetTaskResponse;

use super::entities::{submit_response::SubmitResponse, submit_request::SubmitRequest};

const RABBIT_DEFAULT_PORT: u16 = 5672;
const LOCALHOST: &str = "localhost";

#[derive(Clone)]
pub struct TaskManagerClient {
    connection: Connection,
		app_id: String,
		exchange_name: String,
}

impl TaskManagerClient {

	pub async fn new(exchange_name: &str, connection_string: &str) -> anyhow::Result<Self> {
		let app_id = format!("job-manager.{}", Uuid::new_v4());
		
		let connection = parse_rabbit_connection_string(connection_string).await?;
		
		let queue_client = Self {
			connection,
			app_id,
			exchange_name: exchange_name.to_owned()
		};

		Ok(queue_client)
	}

	pub async fn publish<S, I>(&self, task_name: I, task_parameters: &S) -> anyhow::Result<SubmitResponse>
	where
			S: Serialize,
			I: Into<String>
	{
		let routing_key: String = task_name.into();
		let arguments = BasicPublishArguments::new(&self.exchange_name, &routing_key);
		let channel= self.connection.open_channel(None).await?;
		
		let mut basic_properties = BasicProperties::default();
		basic_properties.with_app_id(&self.app_id);
		
		let submit_request = SubmitRequest::new(task_parameters);
		let task_payload = submit_request.prepare();
		channel.basic_publish(basic_properties, task_payload, arguments).await?;

		let submit_response = submit_request.into_submit_response();

		Ok(submit_response)
	}

	pub async fn get(&self, task_id: &str) -> anyhow::Result<GetTaskResponse> {
		todo!()
	}
}


pub async fn parse_rabbit_connection_string(connection_string: &str) -> anyhow::Result<Connection> {
	let parsed = Url::parse(connection_string)?;
	
	let host = parsed.host_str().unwrap_or(LOCALHOST);
	let port = parsed.port().unwrap_or(RABBIT_DEFAULT_PORT);
	let username = parsed.username();
	let password = parsed.password().unwrap_or_default();

	let options = OpenConnectionArguments::new(host, port, username, password);

	let connection = Connection::open(&options).await?;
	Ok(connection)
} 