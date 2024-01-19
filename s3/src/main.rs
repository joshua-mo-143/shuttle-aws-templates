use axum::{routing::{get, post}, Router};

use shuttle_aws_config::SdkConfig;

mod bucket;
use bucket::BucketWrapper;

mod error;
mod router;
use router::{get_object, add_object, delete_object}; 

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Clone)]
pub struct AppState {
    bucket: BucketWrapper
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_aws_config::Client(
        access_key_id = "{secrets.AWS_ACCESS_KEY_ID}",
        secret_access_key = "{secrets.AWS_SECRET_ACCESS_KEY}",
        region = "eu-west-02"
        )] cfg: SdkConfig,
    ) -> shuttle_axum::ShuttleAxum {
    let bucket = BucketWrapper::new("test_bucket", &cfg);
    let state = AppState { bucket };

    let router = Router::new()
        .route("/", get(hello_world)) 
        .route("/objects/:id", get(get_object).delete(delete_object))
        .route("/objects/create", post(add_object))
        .with_state(state);

    Ok(router.into())
}
