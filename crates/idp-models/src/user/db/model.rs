use sea_orm::{
    DbErr, EntityTrait, FromQueryResult, IdenStatic, IntoActiveModel, ModelTrait, Value,
};

use super::{active_model::ActiveModel, column::Column};

pub type Model = super::super::User;

impl FromQueryResult for Model {
    fn from_query_result(res: &sea_orm::QueryResult, pre: &str) -> Result<Self, DbErr> {
        Ok(Self {
            id: res.try_get(pre, Column::Id.as_str())?,
            email: res.try_get(pre, Column::Email.as_str())?,
            username: res.try_get(pre, Column::Username.as_str())?,
            password: res.try_get(pre, Column::Password.as_str())?,
            session_token: res.try_get(pre, Column::SessionToken.as_str())?,
            created_at: res.try_get(pre, Column::CreatedAt.as_str())?,
        })
    }
}

impl ModelTrait for Model {
    type Entity = super::Entity;

    fn get(&self, entity: <Self::Entity as EntityTrait>::Column) -> Value {
        match entity {
            Column::Id => self.id.to_owned().into(),
            Column::Email => self.email.to_owned().into(),
            Column::Username => self.username.to_owned().into(),
            Column::Password => self.password.to_owned().into(),
            Column::SessionToken => self.session_token.to_owned().into(),
            Column::CreatedAt => self.created_at.to_owned().into(),
        }
    }

    fn set(&mut self, entity: <Self::Entity as EntityTrait>::Column, value: Value) {
        match entity {
            Column::Id => self.id = value.unwrap(),
            Column::Email => self.email = value.unwrap(),
            Column::Username => self.username = value.unwrap(),
            Column::Password => self.password = value.unwrap(),
            Column::SessionToken => self.session_token = value.unwrap(),
            Column::CreatedAt => self.created_at = value.unwrap(),
        }
    }
}

impl IntoActiveModel<ActiveModel> for Model {
    fn into_active_model(self) -> ActiveModel {
        self.into()
    }
}
