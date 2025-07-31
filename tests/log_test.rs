use rlog_dynamo::{debug, error, info, warning};

#[tokio::test]
async fn test_log_macro() {
    dotenv::dotenv().ok();
    let access_key = dotenv::var("AWS_ACCESS_KEY").unwrap();
    let secret_key = dotenv::var("AWS_SECRET_KEY").unwrap();
    let logger = rlog_dynamo::record::DynamoLogger::new(
        "rlog",
        "test",
        access_key,
        secret_key,
        "cn-northwest-1",
        "http://localhost:6667",
        "log_record",
    );

    info!(&logger, "info msg", 0);
    warning!(&logger, &format!("{}", "warn"), 0);
    error!(&logger, "error msg", 0);
    debug!(&logger, "debug msg", 0);
}
