use std::{error::Error, fmt};

use argon2::PasswordHash;
use chrono::{DateTime, Utc};
use cuid2::create_id;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use super::User;

pub struct UserBuilder {
    id: Option<String>,
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    session_token: Option<String>,
    created_at: Option<DateTime<Utc>>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            email: None,
            username: None,
            password: None,
            session_token: None,
            created_at: None,
        }
    }

    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);

        self
    }

    pub fn set_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);

        self
    }

    pub fn set_password(&mut self, password: PasswordHash) -> &mut Self {
        self.password = Some(password.to_string());

        self
    }

    pub fn build(self) -> Result<User, UserBuilderError> {
        let id = self.id.ok_or(UserBuilderError::MissingField)?;

        if !cuid2::is_cuid2(id.clone()) {
            return Err(UserBuilderError::InvalidField);
        }

        let email = self.email.ok_or(UserBuilderError::MissingField)?;
        let username = self.username.ok_or(UserBuilderError::MissingField)?;
        let session_token = self.session_token.ok_or(UserBuilderError::MissingField)?;
        let created_at = self.created_at.ok_or(UserBuilderError::MissingField)?;

        Ok(User {
            id,
            email,
            username,
            password: self.password,
            session_token,
            created_at,
        })
    }
}

impl Default for UserBuilder {
    fn default() -> Self {
        let session_token = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        Self {
            id: Some(create_id()),
            email: None,
            username: None,
            password: None,
            session_token: Some(session_token),
            created_at: Some(Utc::now()),
        }
    }
}

#[derive(Debug)]
pub enum UserBuilderError {
    MissingField,
    InvalidField,
}

impl fmt::Display for UserBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error building User")
    }
}

impl Error for UserBuilderError {}
