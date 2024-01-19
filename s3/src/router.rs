use axum::{extract::{State, Path, Multipart}, http::StatusCode, response::IntoResponse};

use crate::error::ApiError;
use crate::AppState;

pub async fn get_object(
        State(state): State<AppState>,
        Path(key): Path<String>,
    ) -> Result<impl IntoResponse, ApiError> {
   let object = state.bucket.get_object(&key).await?;

   let bytes: Vec<u8> = object.body.collect().await.unwrap().into_bytes().to_vec(); 

   Ok(bytes)
} 

pub async fn add_object(
        State(state): State<AppState>,
        mut multipart: Multipart
    ) -> Result<impl IntoResponse, ApiError> {
    let mut filename: Option<String> = None;
    let mut data: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
      filename = Some(field.name().unwrap().to_string());
      data = Some(field.bytes().await.unwrap().to_vec());
    }

   let _object = state.bucket.add_object(&filename.unwrap(), data.unwrap()).await?;

   Ok(StatusCode::OK)
} 

pub async fn delete_object(
        State(state): State<AppState>,
        Path(key): Path<String>,
    ) -> Result<StatusCode, ApiError> {
   state.bucket.delete_object(&key).await?;

   Ok(StatusCode::OK) 
} 
