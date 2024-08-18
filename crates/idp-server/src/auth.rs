use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    #[serde(rename = "eat")]
    expires_at: i64,
    #[serde(rename = "sub")]
    subject: String,
}

impl Claims {
    pub fn new(expiry: DateTime<Utc>, subject: String) -> Self {
        Self {
            expires_at: expiry.timestamp(),
            subject,
        }
    }

    pub fn expiry(&self) -> DateTime<Utc> {
        let timestamp = self.expires_at;

        match DateTime::from_timestamp(timestamp, 0) {
            Some(a) => a,
            None => Utc::now(),
        }
    }

    pub fn subject(&self) -> &String {
        &self.subject
    }
}
