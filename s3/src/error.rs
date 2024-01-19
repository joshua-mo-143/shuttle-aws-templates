use axum::{http::StatusCode, response::{Response, IntoResponse}};
use aws_sdk_s3::{error::SdkError,
    operation::put_object::PutObjectError, 
    operation::get_object::GetObjectError,
    operation::delete_object::DeleteObjectError,
    primitives::ByteStreamError
};

pub enum ApiError {
    PutError(SdkError<PutObjectError>),
    GetError(SdkError<GetObjectError>),
    DeleteError(SdkError<DeleteObjectError>),
    BucketIOError(ByteStreamError)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::PutError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::GetError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::DeleteError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::BucketIOError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

impl From<SdkError<PutObjectError>> for ApiError {
    fn from(e: SdkError<PutObjectError>) -> Self {
        Self::PutError(e)
    }
}
impl From<SdkError<GetObjectError>> for ApiError {
    fn from(e: SdkError<GetObjectError>) -> Self {
        Self::GetError(e)
    }
}
impl From<SdkError<DeleteObjectError>> for ApiError {
    fn from(e: SdkError<DeleteObjectError>) -> Self {
        Self::DeleteError(e)
    }
}

impl From<ByteStreamError> for ApiError {
    fn from(e: ByteStreamError) -> Self {
        Self::BucketIOError(e)
    }
}
