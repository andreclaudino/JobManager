pub mod internal_error;
pub mod entities;
pub mod routes;
use actix_web::{HttpServer, App, web};

use crate::task_manager::client::TaskManagerClient;

use self::routes::make_task_scope;

pub async fn start_server(manager_client: TaskManagerClient, host: &str, port: &u16) -> anyhow::Result<()> {
	let bind_address = (host, *port);
	let queue_client_data = web::Data::new(manager_client);

	HttpServer::new(move || {
		App::new()
			.app_data(queue_client_data.clone())
			.service(make_task_scope())
	})
	.bind(bind_address)?
	.run()
	.await?;

	Ok(())
}