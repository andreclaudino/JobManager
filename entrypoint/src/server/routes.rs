use crate::task_manager::client::TaskManagerClient;
use actix_web::{web, HttpResponse, Scope};

use super::{
    entities::{
        get_task_parameters::GetTaskResponse, send_task_parameters::SendTaskParameters,
        send_task_payload::SendTaskPayload,
    },
    internal_error::InternalError,
};

pub fn make_task_scope() -> Scope {
    web::scope("/task").service(
        web::resource("/{task_name}")
            .route(web::post().to(send_task))
            .route(web::get().to(get_task)),
    )
}

pub async fn send_task(
    manager_client_: web::Data<TaskManagerClient>,
    path_parameters_: web::Path<SendTaskParameters>,
    task_parameters: web::Json<SendTaskPayload>,
) -> actix_web::Result<HttpResponse> {
    let manager_client = manager_client_.into_inner();
    let path_parameters = path_parameters_.into_inner();

    let task_name = path_parameters.task_name;

    let task_submit_response = manager_client
        .publish(task_name, &task_parameters)
        .await
        .map_err(InternalError::from)?;

    let response = HttpResponse::Created().json(&task_submit_response);

    Ok(response)
}

pub async fn get_task(
    manager_client_: web::Data<TaskManagerClient>,
    path_parameters_: web::Path<GetTaskResponse>,
) -> actix_web::Result<HttpResponse> {
    let manager_client = manager_client_.into_inner();
    let path_parameters = path_parameters_.into_inner();

    let task_id = path_parameters.task_id;

    let task_response = manager_client
        .get(&task_id)
        .await
        .map_err(InternalError::from)?;

    let response = HttpResponse::Created().json(&task_response);

    Ok(response)
}
