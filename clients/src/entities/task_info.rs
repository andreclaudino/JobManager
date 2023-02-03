use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::publish_response::PublishRequest;


#[derive(Serialize, Deserialize, Default)]
pub struct TaskInfo {
	pub task_status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submited_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,
}

impl TaskInfo {
	pub fn maformed() -> Self {
		Self {
			task_status: TaskStatus::Failed{message: "Cant parse returned value".to_owned()},
			..Self::default()
		}
	}}

impl From<PublishRequest> for TaskInfo {
    fn from(value: PublishRequest) -> Self {
        TaskInfo {
            task_status: TaskStatus::Pending,
            submited_at: Some(value.submited_at),
            ..TaskInfo::default()
        }
    }
}


#[derive(Serialize, Deserialize)]
#[serde(rename="UPPERCASE")]
pub enum TaskStatus {
    Pending,
    Waiting,
	Succeed{result: serde_json::Value},
    Failed{message: String},
    NotFound,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Waiting
    }
}

impl Into<u8> for TaskStatus {
    fn into(self) -> u8 {
        Into::<TaskStatusCode>::into(self).into()
    }
}

pub enum TaskStatusCode {
    Pending,
    Waiting,
	Succeed,
    Failed,
    Invalid
}

impl From<u8> for TaskStatusCode {
    fn from(code: u8) -> Self {
        match code {
            1 => Self::Pending,
            2 => Self::Waiting,
            3 => Self::Succeed,
            4 => Self::Failed,
            _ => Self::Invalid,
        }
    }

}

impl Into<u8> for TaskStatusCode {
    fn into(self) -> u8 {
        match self {
            TaskStatusCode::Pending => 1,
            TaskStatusCode::Waiting => 2,
            TaskStatusCode::Succeed => 3,
            TaskStatusCode::Failed => 4,
            TaskStatusCode::Invalid => 0
        }
    }
}

impl Into<TaskStatusCode> for TaskStatus {
    fn into(self) -> TaskStatusCode {
        match self {
            Self::Pending => TaskStatusCode::Pending,
            Self::Waiting => TaskStatusCode::Waiting,
            Self::Succeed{..} => TaskStatusCode::Succeed,
            Self::Failed{..} => TaskStatusCode::Failed,
            Self::NotFound => TaskStatusCode::Invalid,
        }
    }
}
