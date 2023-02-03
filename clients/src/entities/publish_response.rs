use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct PublishRequest {
    pub task_id: Uuid,
    pub submited_at: DateTime<Utc>,
    pub task_parameters: HashMap<String, serde_json::Value>,
}


impl PublishRequest {
    pub fn new(task_parameters: &HashMap<String, serde_json::Value>) -> PublishRequest {
        Self {
            task_id: Uuid::new_v4(),
            submited_at: Utc::now(),
            task_parameters: task_parameters.to_owned()
        }
    }

    pub fn prepare(&self) -> Vec<u8> {
        let prepared = bincode::serialize(self).unwrap();
        prepared
    }
}
