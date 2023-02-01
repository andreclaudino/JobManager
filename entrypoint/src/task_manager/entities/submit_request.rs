use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::submit_response::SubmitResponse;

#[derive(Serialize, Deserialize)]
pub struct SubmitRequest<S>
where
    S: Serialize,
{
    task_id: Uuid,
    submited_at: DateTime<Utc>,
    payload: S,
}

impl<S> SubmitRequest<S>
where
    S: Serialize,
{
    pub fn new(payload: S) -> Self {
        Self {
            task_id: Uuid::new_v4(),
            submited_at: Utc::now(),
            payload,
        }
    }

    pub fn into_submit_response(&self) -> SubmitResponse {
        SubmitResponse {
            task_id: self.task_id,
            submited_at: self.submited_at,
        }
    }

    pub fn prepare(&self) -> Vec<u8> {
        let prepared = bincode::serialize(self).unwrap();
        prepared
    }
}
