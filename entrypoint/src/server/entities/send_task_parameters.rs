use serde::Deserialize;


#[derive(Deserialize)]
pub struct SendTaskParameters {
	pub task_name: String,
}