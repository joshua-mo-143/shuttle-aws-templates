use aws_sdk_s3::{primitives::{ByteStream, SdkBody}, 
    Client,
    operation::put_object::{PutObjectOutput, PutObjectError},
    operation::get_object::{GetObjectOutput, GetObjectError},
    operation::delete_object::DeleteObjectError,
    error::SdkError 
};
use shuttle_aws_config::SdkConfig;

#[derive(Clone)]
pub struct BucketWrapper {
   bucket_name: String,
   ctx: Client
}

impl BucketWrapper {
    pub fn new(bucket_name: &str, cfg: &SdkConfig) -> Self {
        let ctx = Client::new(cfg);
        Self {
            bucket_name: bucket_name.to_string(), 
            ctx
        }
    }

    pub async fn get_object(&self, key: &str) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
        self.ctx
            .get_object()
            .bucket(self.bucket_name())
            .key(key)
            .send()
            .await
    }

    pub async fn add_object(&self, key: &str, data: Vec<u8>) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        let body = ByteStream::new(SdkBody::from(&*data));

        self.ctx
            .put_object()
            .bucket(self.bucket_name())
            .key(key)
            .body(body)
            .send()
            .await
    }

    pub async fn delete_object(&self, key: &str) -> Result<(), SdkError<DeleteObjectError>> {
        self.ctx
            .delete_object()
            .bucket(self.bucket_name())
            .key(key)
            .send()
            .await?;

        println!("Object was deleted.");

        Ok(())
    }
}

impl<'a> BucketWrapper {
    pub fn bucket_name(&'a self) -> &'a str {
        &self.bucket_name
    }
}
