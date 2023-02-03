use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SendTaskPayload {
    pub task_parameters: HashMap<String, serde_json::Value>,
}