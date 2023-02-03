use serde::Deserialize;


#[derive(Deserialize)]
pub struct SendTaskParameters {
	pub task_function_name: String,
}

#[derive(Deserialize)]
pub struct GetTaskParameters {
	pub task_id: String,
}