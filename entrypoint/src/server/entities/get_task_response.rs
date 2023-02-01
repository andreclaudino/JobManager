use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct TaskResponse {
	pub task_id: String,
	pub result: TaskResult,
}


#[derive(Serialize, Deserialize)]
#[serde(rename="UPPERCASE")]
pub enum TaskResult {
	Succeed,
	Waiting,
	Failed,
}
