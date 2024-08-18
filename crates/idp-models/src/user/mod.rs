mod builder;
mod db;

use std::fmt;
pub use db::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation};
pub use builder::UserBuilder;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct User {
    id: String,
    email: String,
    username: String,
    password: Option<String>, // PHC String
    session_token: String,
    created_at: DateTime<Utc>,
}

impl User {
    pub fn builder() -> UserBuilder {
        UserBuilder::new()
    }

    pub fn validate_password(&self, _password: String) -> bool {
        let _ = &self.password;
        todo!("This function should return a Result<(), UserAuthenticationError> and actually check the users credentials.")
    }

    pub fn validate_session_token(&self, _session_token: String) -> bool {
        let _ = &self.session_token;
        todo!("This function should return a Result<> and check the session_token")
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

// impl Default for User {
//     fn default() -> Self {
//         Self {
//             id: Default::default(),
//             email: Default::default(),
//             username: Default::default(),
//             password: Default::default(),
//             session_token: Default::default(),
//             created_at: Default::default(),
//         }
//     }
// }

impl fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .field("session_token", &"[REDACTED]")
            .field("created_at", &self.created_at)
            .finish()
    }
}
