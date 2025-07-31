use crate::{models, utils};
use aws_sdk_dynamodb::Client;
use serde::{Deserialize, Serialize};
use serde_dynamo::aws_sdk_dynamodb_1::to_item;

#[macro_export]
macro_rules! info {
    ($logger:expr, $log:expr, $user:expr) => {{
        let func_name = $crate::func_name!();
        $logger.info($log, $user, func_name).await
    }};
}

#[macro_export]
macro_rules! warning {
    ($logger:expr, $log:expr, $user:expr) => {{
        let func_name = $crate::func_name!();
        $logger.warning($log, $user, func_name).await
    }};
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $log:expr, $user:expr) => {{
        let func_name = $crate::func_name!();
        $logger.error($log, $user, func_name).await
    }};
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $log:expr, $user:expr) => {{
        let func_name = $crate::func_name!();
        $logger.debug($log, $user, func_name).await
    }};
}

pub struct DynamoLogger {
    project: &'static str,
    app: &'static str,
    // access_key: &'static str,
    // secret_key: &'static str,
    // endpoint_url: &'static str,
    table_name: &'static str,
    client: Client,
}
impl DynamoLogger {
    pub fn new(
        project: &'static str,
        app: &'static str,
        access_key: String,
        secret_key: String,
        region: &'static str,
        endpoint_url: &'static str,
        table_name: &'static str,
    ) -> Self {
        let client = utils::build_dynamo_client(access_key, secret_key, region, endpoint_url);
        Self {
            project,
            app,
            // access_key,
            // secret_key,
            // endpoint_url,
            table_name,
            client,
        }
    }

    pub async fn log_with_level(&self, log: &str, user: i64, level: models::LogLevel, func: &str) {
        let record = NewLogRecord::new(self.app, func, log, self.project, user, level);
        let item = to_item(record).unwrap();
        self.client
            .put_item()
            .table_name(self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .unwrap();
    }

    pub async fn info(&self, log: &str, user: i64, func: &str) {
        self.log_with_level(log, user, models::LogLevel::Info, func)
            .await;
    }
    pub async fn warning(&self, log: &str, user: i64, func: &str) {
        self.log_with_level(log, user, models::LogLevel::Warning, func)
            .await;
    }
    pub async fn error(&self, log: &str, user: i64, func: &str) {
        self.log_with_level(log, user, models::LogLevel::Error, func)
            .await;
    }
    pub async fn debug(&self, log: &str, user: i64, func: &str) {
        self.log_with_level(log, user, models::LogLevel::Debug, func)
            .await;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewLogRecord {
    pub pk: String,
    pub sk: String,
    pub app: &'static str,
    pub func: String,
    pub log: String,
    pub project: &'static str,
    pub user: i64,
    pub level: models::LogLevel,
    pub created: chrono::DateTime<chrono::Utc>,
}
impl NewLogRecord {
    pub fn new(
        app: &'static str,
        func: &str,
        log: &str,
        project: &'static str,
        user: i64,
        level: models::LogLevel,
    ) -> Self {
        let utc_now = chrono::Utc::now();
        let day_part = utc_now.format("%Y-%m-%d").to_string();
        let time_part = utc_now.format("%H:%M:%S:%6f").to_string();
        let pk = format!("{}#{}", project, day_part);
        let sk = format!("{}#{}#{}", time_part, func, user);
        Self {
            pk,
            sk,
            app,
            func: func.to_string(),
            log: log.to_string(),
            project,
            user,
            level,
            created: utc_now,
        }
    }
}
