use aws_config::Region;
use aws_sdk_dynamodb::config::Credentials;
use aws_sdk_dynamodb::{Client, Config};

#[macro_export]
macro_rules! func_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let arr: Vec<&str> = type_name_of(f).split("::").collect();
        arr[arr.len() - 3]
    }};
}
#[macro_export]
macro_rules! file_name {
    () => {{
        use std::path::Path;
        let file_name = Path::new(file!()).file_name().unwrap().to_str().unwrap();
        let file_name_without_ext = file_name.strip_suffix(".rs").unwrap_or(file_name);
        file_name_without_ext
    }};
}

pub fn build_dynamo_client(
    access_key: String,
    secret_key: String,
    region: &str,
    endpoint_url: &str,
) -> Client {
    let credentials = Credentials::new(access_key, secret_key, None, None, "manual");
    let config = if endpoint_url.is_empty() {
        Config::builder()
            .region(Region::new(region.to_string()))
            .credentials_provider(credentials)
            .build()
    } else {
        Config::builder()
            .region(Region::new(region.to_string()))
            .credentials_provider(credentials)
            .endpoint_url(endpoint_url)
            .build()
    };

    Client::from_conf(config)
}
