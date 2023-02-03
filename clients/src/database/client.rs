use std::sync::Arc;
use chrono::{DateTime, Utc, NaiveDateTime};
use redis::{aio::Connection, AsyncCommands, FromRedisValue, ToRedisArgs};
use tokio::sync::Mutex;

use crate::entities::task_info::{TaskInfo, TaskStatusCode, TaskStatus};

pub struct DatabaseClient {
    redis_connection: Arc<Mutex<Connection>>,
}

impl DatabaseClient {
    pub async fn new(redis_connection_str: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(redis_connection_str)?;
        let redis_connection = Arc::new(Mutex::new(client.get_async_connection().await?));

        Ok(Self { redis_connection })
    }

    pub async fn upset_task_result(&self, task_id: &str, task_data: TaskInfo) -> anyhow::Result<()> {
        let mut connection = self.redis_connection.lock().await;
        let task_status_code: u8 = task_data.task_status.into();

        write_key::<u8>(&mut connection, &task_id, "status", task_status_code).await?;

        if let Some(started_at) = task_data.started_at {
            write_date_key(&mut connection, &task_id, "started_at", started_at).await?;
        }

        if let Some(submited_at) = task_data.submited_at {
            write_date_key(&mut connection, &task_id, "submited_at", submited_at).await?;
        }

        if let Some(finished_at) = task_data.finished_at {
            write_date_key(&mut connection, &task_id, "status", finished_at).await?;
        }

        Ok(())
    }

    pub async fn get_task_result(&self, task_id: &str) -> anyhow::Result<TaskInfo> {
        let mut connection = self.redis_connection.lock().await;
        
        let status_code: Option<TaskStatusCode> = read_key::<u8>(&mut connection, task_id, "status").await?.map(TaskStatusCode::from);

        let task_response =
            match status_code {
                Some(TaskStatusCode::Pending) => {
                    let submited_at = read_date_key(&mut connection, task_id, "submited_at").await?;

                    TaskInfo {
                        task_status: TaskStatus::Pending,
                        submited_at,
                        ..TaskInfo::default()
                    }
                },
                Some(TaskStatusCode::Waiting) => {
                    let submited_at = read_date_key(&mut connection, task_id, "submited_at").await?;
                    let started_at = read_date_key(&mut connection, task_id, "started_at").await?;

                    TaskInfo {
                        task_status: TaskStatus::Waiting,
                        submited_at,
                        started_at,
                        ..TaskInfo::default()
                    }
                },
                Some(TaskStatusCode::Succeed) => {
                    let content: Option<Vec<u8>> = read_key(&mut connection, task_id, "content").await?;
                    let submited_at = read_date_key(&mut connection, task_id, "submited_at").await?;
                    let started_at = read_date_key(&mut connection, task_id, "started_at").await?;
                    
                    TaskInfo {
                        task_status: TaskStatus::Succeed(content.unwrap_or_default()),
                        submited_at,
                        started_at,
                        ..TaskInfo::default()
                    }
                },
                Some(TaskStatusCode::Failed) => {
                    let error: Option<String> = read_key(&mut connection, task_id, "content").await?;
                    let submited_at = read_date_key(&mut connection, task_id, "submited_at").await?;
                    let started_at = read_date_key(&mut connection, task_id, "started_at").await?;
                    
                    TaskInfo {
                        task_status: TaskStatus::Failed(error.unwrap_or_default()),
                        submited_at,
                        started_at,
                        ..TaskInfo::default()
                    }
                },
                _ => 
                TaskInfo {
                    task_status: TaskStatus::NotFound,
                    ..TaskInfo::default()
                },
            };
        Ok(task_response)
    }    
}

pub async fn read_key<T: FromRedisValue>(connection: &mut Connection, task_id: &str, feature: &str) -> anyhow::Result<Option<T>> {
    let key = format!("{task_id}.{feature}");
    let response: Option<T> = connection.get(&key).await?;
    Ok(response)
}

pub async fn write_key<T: ToRedisArgs + Sync + Send>(connection: &mut Connection, task_id: &str, feature: &str, value: T) -> anyhow::Result<()> {
    let key = format!("{task_id}.{feature}");
    connection.set(&key, value).await?;
    Ok(())
}

pub async fn read_date_key(connection: &mut Connection, task_id: &str, feature: &str) -> anyhow::Result<Option<DateTime<Utc>>> {
    let key = format!("{task_id}.{feature}");
    let unix_timestamp: Option<i64> = connection.get(&key).await?;
    let response = unix_timestamp.map(|value| {
        let naive_date_time = NaiveDateTime::from_timestamp_millis(value).unwrap();
        DateTime::from_utc(naive_date_time, Utc)
    });
    Ok(response)
}


pub async fn write_date_key(connection: &mut Connection, task_id: &str, feature: &str, value: DateTime<Utc>) -> anyhow::Result<()> {
    let key = format!("{task_id}.{feature}");
    
    let unix_timestamp: i64 = value.timestamp();
    connection.set(&key, unix_timestamp).await?;
    Ok(())
}