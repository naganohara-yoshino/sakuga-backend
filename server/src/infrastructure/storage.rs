use crate::config::AppConfig;
use crate::error::AppResult;
use opendal::Operator;
use opendal::services::S3;

pub async fn create_operator(config: &AppConfig) -> AppResult<Operator> {
    let builder = S3::default()
        .bucket(&config.s3_bucket)
        .region(&config.s3_region)
        .endpoint(&config.s3_endpoint)
        .access_key_id(&config.s3_access_key_id)
        .secret_access_key(&config.s3_secret_access_key)
        .delete_max_size(600);

    let op = Operator::new(builder)?.finish();

    Ok(op)
}
