pub mod internal_error;
pub mod entities;
pub mod routes;
use actix_web::{HttpServer, App, web};
use clients::{QueueClient, DatabaseClient};

use self::routes::make_task_scope;

pub async fn start_server(manager_client: QueueClient, database_client: DatabaseClient, host: &str, port: &u16) -> anyhow::Result<()> {
	let bind_address = (host.clone(), *port);

	let queue_client_data = web::Data::new(manager_client);
	let database_client_data = web::Data::new(database_client);

	log::info!("Starting server on {host}:{port}", host=host, port=port);

	HttpServer::new(move || {
		App::new()
			.app_data(queue_client_data.clone())
			.app_data(database_client_data.clone())
			.service(make_task_scope())
	})
	.bind(bind_address)?
	.run()
	.await?;

	Ok(())
}