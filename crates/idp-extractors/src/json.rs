use axum::{
    async_trait,
    body::Bytes,
    extract::{rejection::BytesRejection, FromRequest, Request},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response,
};
use bytes::{BufMut as _, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub struct Json<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // If Content-Type is not application/json
        if !Self::valid_content_type(req.headers()) {
            // Return HTTP 415 Unsupported Media Type
            return Err(JsonRejection::InvalidContentType);
        }

        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|e| JsonRejection::BytesRejection(e))?;

        Self::from_bytes(&bytes)
    }
}

impl<T> Json<T> {
    fn valid_content_type(headers: &HeaderMap) -> bool {
        let content_type = if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
            content_type
        } else {
            return false;
        };

        let content_type = if let Ok(content_type) = content_type.to_str() {
            content_type
        } else {
            return false;
        };

        let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
            mime
        } else {
            return false;
        };

        let is_json_content_type = mime.type_() == "application"
            && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"));

        is_json_content_type
    }
}

impl<T> Json<T>
where
    T: DeserializeOwned,
{
    // Fully pinched from Axum's built-in Json extractor.
    // https://github.com/tokio-rs/axum/blob/main/axum/src/json.rs
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JsonRejection> {
        let deserializer = &mut serde_json::Deserializer::from_slice(bytes);

        let value = match serde_path_to_error::deserialize(deserializer) {
            Ok(value) => value,
            Err(err) => {
                let rejection = match err.inner().classify() {
                    serde_json::error::Category::Data => {
                        // JsonRejection::JsonDataError::from_err(err).into()
                        JsonRejection::JsonDataError
                    }
                    serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                        // JsonRejection::JsonSyntaxError::from_err(err).into()
                        JsonRejection::JsonSyntaxError
                    }
                    serde_json::error::Category::Io => {
                        if cfg!(debug_assertions) {
                            // we don't use `serde_json::from_reader` and instead always buffer
                            // bodies first, so we shouldn't encounter any IO errors
                            unreachable!()
                        }

                        // JsonRejection::JsonSyntaxError::from_err(err).into()
                        JsonRejection::JsonSyntaxError
                    }
                };
                return Err(rejection);
            }
        };

        Ok(Json(value))
    }
}

impl<T> response::IntoResponse for Json<T>
where
    T: Serialize,
{
    // Fully pinched from Axum's built-in Json extractor.
    // https://github.com/tokio-rs/axum/blob/main/axum/src/json.rs
    fn into_response(self) -> response::Response {
        // Use a small initial capacity of 128 bytes like serde_json::to_vec
        // https://docs.rs/serde_json/1.0.82/src/serde_json/ser.rs.html#2189
        let mut buf = BytesMut::with_capacity(128).writer();
        match serde_json::to_writer(&mut buf, &self.0) {
            Ok(()) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                buf.into_inner().freeze(),
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

#[derive(Debug, Error)]
pub enum JsonRejection {
    #[error("Request body could not be deserialized into T")]
    JsonDataError,
    #[error("Request body presented malformed JSON")]
    JsonSyntaxError,
    #[error("Failed to parse request into bytes")]
    BytesRejection(BytesRejection),
    #[error("Request body did not contain valid JSON Content-Type")]
    InvalidContentType,
}

impl response::IntoResponse for JsonRejection {
    fn into_response(self) -> response::Response {
        match self {
            Self::JsonDataError => StatusCode::BAD_REQUEST,
            Self::JsonSyntaxError => StatusCode::UNPROCESSABLE_ENTITY,
            Self::BytesRejection(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidContentType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        }
        .into_response()
    }
}
