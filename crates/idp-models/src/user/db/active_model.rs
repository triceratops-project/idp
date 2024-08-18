use std::mem;

use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ActiveValue, Value};

use super::{column::Column, model::Model, Entity};

#[derive(Clone, Debug)]
pub struct ActiveModel {
    id: ActiveValue<String>,
    email: ActiveValue<String>,
    username: ActiveValue<String>,
    password: ActiveValue<Option<String>>,
    session_token: ActiveValue<String>,
    created_at: ActiveValue<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModelTrait for ActiveModel {
    type Entity = Entity;

    fn take(&mut self, c: Column) -> ActiveValue<Value> {
        match c {
            Column::Id => {
                let mut value = ActiveValue::not_set();
                mem::swap(&mut value, &mut self.id);
                value.into_wrapped_value()
            }
            Column::Email => {
                let mut value = ActiveValue::not_set();
                mem::swap(&mut value, &mut self.email);
                value.into_wrapped_value()
            }
            Column::Username => {
                let mut value = ActiveValue::not_set();
                mem::swap(&mut value, &mut self.username);
                value.into_wrapped_value()
            }
            Column::Password => {
                let mut value = ActiveValue::not_set();
                mem::swap(&mut value, &mut self.password);
                value.into_wrapped_value()
            }
            Column::SessionToken => {
                let mut value = ActiveValue::not_set();
                mem::swap(&mut value, &mut self.session_token);
                value.into_wrapped_value()
            }
            Column::CreatedAt => {
                let mut value = ActiveValue::not_set();
                mem::swap(&mut value, &mut self.created_at);
                value.into_wrapped_value()
            }
        }
    }

    fn get(&self, c: Column) -> ActiveValue<Value> {
        match c {
            Column::Id => self.id.clone().into_wrapped_value(),
            Column::Email => self.email.clone().into_wrapped_value(),
            Column::Username => self.username.clone().into_wrapped_value(),
            Column::Password => self.password.clone().into_wrapped_value(),
            Column::SessionToken => self.session_token.clone().into_wrapped_value(),
            Column::CreatedAt => self.created_at.clone().into_wrapped_value(),
        }
    }

    fn set(&mut self, c: Column, v: Value) {
        match c {
            Column::Id => self.id = ActiveValue::set(v.unwrap()),
            Column::Email => self.email = ActiveValue::set(v.unwrap()),
            Column::Username => self.username = ActiveValue::set(v.unwrap()),
            Column::Password => self.password = ActiveValue::set(v.unwrap()),
            Column::SessionToken => self.session_token = ActiveValue::set(v.unwrap()),
            Column::CreatedAt => self.created_at = ActiveValue::set(v.unwrap()),
        }
    }

    fn not_set(&mut self, c: Column) {
        match c {
            Column::Id => self.id = ActiveValue::not_set(),
            Column::Email => self.email = ActiveValue::not_set(),
            Column::Username => self.username = ActiveValue::not_set(),
            Column::Password => self.password = ActiveValue::not_set(),
            Column::SessionToken => self.session_token = ActiveValue::not_set(),
            Column::CreatedAt => self.created_at = ActiveValue::not_set(),
        }
    }

    fn is_not_set(&self, c: Column) -> bool {
        match c {
            Column::Id => self.id.is_not_set(),
            Column::Email => self.email.is_not_set(),
            Column::Username => self.username.is_not_set(),
            Column::Password => self.password.is_not_set(),
            Column::SessionToken => self.session_token.is_not_set(),
            Column::CreatedAt => self.created_at.is_not_set(),
        }
    }

    fn default() -> Self {
        Self {
            id: ActiveValue::NotSet,
            email: ActiveValue::NotSet,
            username: ActiveValue::NotSet,
            password: ActiveValue::NotSet,
            session_token: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
        }
    }

    fn reset(&mut self, c: Column) {
        match c {
            Column::Id => self.id.reset(),
            Column::Email => self.email.reset(),
            Column::Username => self.username.reset(),
            Column::Password => self.password.reset(),
            Column::SessionToken => self.session_token.reset(),
            Column::CreatedAt => self.created_at.reset(),
        }
    }
}

impl From<Model> for ActiveModel {
    fn from(model: Model) -> Self {
        Self {
            id: ActiveValue::unchanged(model.id),
            email: ActiveValue::unchanged(model.email),
            username: ActiveValue::unchanged(model.username),
            password: ActiveValue::unchanged(model.password),
            session_token: ActiveValue::unchanged(model.session_token),
            created_at: ActiveValue::unchanged(model.created_at),
        }
    }
}
