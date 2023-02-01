use serde::{Serialize};


#[derive(Serialize)]
pub struct GetTaskResponse {
	pub task_id: String,
	pub content: Vec<u8>,
}