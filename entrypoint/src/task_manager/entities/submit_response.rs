use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct SubmitResponse {
    pub(super) task_id: Uuid,
    pub(super) submited_at: DateTime<Utc>,
}
