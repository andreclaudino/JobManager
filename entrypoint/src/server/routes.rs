use actix_web::{web, HttpResponse, Scope};
use clients::{QueueClient, DatabaseClient, entities::task_info::TaskInfo};

use super::{
    entities::{
        send_task_parameters::{SendTaskParameters, GetTaskParameters},
        send_task_payload::SendTaskPayload,
    },
    internal_error::InternalError,
};

pub fn make_task_scope() -> Scope {
    web::scope("/task")
        .service(web::resource("/{task_function_name}").route(web::post().to(send_task)))
        .service(web::resource("/{task_id}").route(web::get().to(get_task)))
        
}

pub async fn send_task(queue_client_: web::Data<QueueClient>, database_client_: web::Data<DatabaseClient>,
    path_parameters_: web::Path<SendTaskParameters>, payload_: web::Json<SendTaskPayload>) -> actix_web::Result<HttpResponse> {
    
    let queue_client = queue_client_.into_inner();
    let path_parameters = path_parameters_.into_inner();
    let task_name = path_parameters.task_function_name;

    let payload = payload_.into_inner();
    let task_parameters = payload.task_parameters;

    log::info!("Submit request received for {task_name}");

    let task_submit_response =
        queue_client
            .publish(task_name, &task_parameters)
                .await
                    .map_err(InternalError::from)?;
    
    let task_id = task_submit_response.task_id.clone().to_string();
    let task_info: TaskInfo = task_submit_response.clone().into();
    
    database_client_.upset_task_result(&task_id, task_info).await.map_err(InternalError::from)?;

    let response = HttpResponse::Created().json(&task_submit_response);
    Ok(response)
}


pub async fn get_task(database_client_: web::Data<DatabaseClient>, path_parameters_: web::Path<GetTaskParameters>) -> actix_web::Result<HttpResponse> {
    let database_client = database_client_.into_inner();
    let path_parameters = path_parameters_.into_inner();
    let task_id = path_parameters.task_id;

    let task_info =
        database_client
            .get_task_result(&task_id)
            .await
            .map_err(InternalError::from)?;
    
    let response = HttpResponse::Created().json(&task_info);
    Ok(response)
}
