use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct GetTaskResponse {
	pub task_id: String,
}