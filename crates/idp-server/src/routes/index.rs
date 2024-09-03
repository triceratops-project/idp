use axum::Json;
use serde::Serialize;

pub struct StatusHandler;

#[derive(Debug, Serialize)]
pub struct StatusResponse<'a> {
    pub status: &'a str,
}

impl StatusHandler {
    pub async fn get<'a>() -> Json<StatusResponse<'a>> {
        Json(StatusResponse { status: "Online" })
    }
}
