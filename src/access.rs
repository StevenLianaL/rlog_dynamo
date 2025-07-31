use crate::utils;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue};
use std::collections::HashMap;

pub struct LogAccess {
    project: &'static str,
    table_name: &'static str,
    client: Client,
}
impl LogAccess {
    pub fn new(
        project: &'static str,
        access_key: String,
        secret_key: String,
        region: &'static str,
        endpoint_url: &'static str,
        table_name: &'static str,
    ) -> Self {
        let client = utils::build_dynamo_client(access_key, secret_key, region, endpoint_url);
        Self {
            project,
            table_name,
            client,
        }
    }
    pub async fn increase(&self, app: &str, action: &str) {
        let utc_now = chrono::Utc::now();
        let month_part = utc_now.format("%Y-%m").to_string();
        let pk = format!("{}#{}", self.project, month_part);
        // 构建 Key
        let mut key = HashMap::new();
        key.insert("pk".to_string(), AttributeValue::S(pk));
        key.insert("action".to_string(), AttributeValue::S(action.to_string()));

        // 构建 ExpressionAttributeNames
        let mut expression_attribute_names = HashMap::new();
        expression_attribute_names.insert("#c".to_string(), "count".to_string());
        expression_attribute_names.insert("#a".to_string(), "app".to_string());
        expression_attribute_names.insert("#m".to_string(), "month".to_string());

        // 构建 ExpressionAttributeValues
        let mut expression_attribute_values = HashMap::new();
        expression_attribute_values
            .insert(":start".to_string(), AttributeValue::N("0".to_string()));
        expression_attribute_values.insert(":inc".to_string(), AttributeValue::N("1".to_string()));
        expression_attribute_values.insert(":app".to_string(), AttributeValue::S(app.to_string()));
        expression_attribute_values.insert(":month".to_string(), AttributeValue::S(month_part.to_string()));

        // 执行 update_item
        self.client
            .update_item()
            .table_name(self.table_name)
            .set_key(Some(key))
            .update_expression("SET #c = if_not_exists(#c, :start) + :inc, #a = :app, #m = :month")
            .set_expression_attribute_names(Some(expression_attribute_names))
            .set_expression_attribute_values(Some(expression_attribute_values))
            .return_values(ReturnValue::UpdatedNew)
            .send()
            .await
            .unwrap();
    }
}
