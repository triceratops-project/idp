use std::str::FromStr;

use sea_orm::{ColumnFromStrErr, ColumnTrait, ColumnType, ColumnTypeTrait, EnumIter, Iden, IdenStatic};

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Column {
    Id,
    Email,
    Username,
    Password,
    SessionToken,
    CreatedAt,
}

impl Column {
    fn default_as_str(&self) -> &str {
        match self {
            Self::Id => "id",
            Self::Email => "email",
            Self::Username => "username",
            Self::Password => "password",
            Self::SessionToken => "session_token",
            Self::CreatedAt => "created_at",
        }
    }
}

impl ColumnTrait for Column {
    type EntityName = super::Entity;

    fn def(&self) -> sea_orm::ColumnDef {
        match self {
            Self::Id => ColumnTypeTrait::def(ColumnType::string(None)),
            Self::Email => ColumnTypeTrait::def(ColumnType::string(None)),
            Self::Username => ColumnTypeTrait::def(ColumnType::string(None)),
            Self::Password => ColumnTypeTrait::def(ColumnType::string(None)),
            Self::SessionToken => ColumnTypeTrait::def(ColumnType::string(None)),
            Self::CreatedAt => ColumnTypeTrait::def(ColumnType::Timestamp),
        }
    }
}

impl FromStr for Column {
    type Err = ColumnFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "id" => Ok(Self::Id),
            "email" => Ok(Self::Email),
            "username" => Ok(Self::Username),
            "password" => Ok(Self::Password),
            "session_token" | "sessionToken" => Ok(Self::SessionToken),
            "created_at" | "createdAt" => Ok(Self::CreatedAt),
            _ => Err(ColumnFromStrErr(s.to_owned())),
        }
    }
}

impl IdenStatic for Column {
    fn as_str(&self) -> &str {
        self.default_as_str()
    }
}

impl Iden for Column {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        s.write_fmt(format_args!("{0}", sea_orm::IdenStatic::as_str(self)))
            .unwrap();
    }
}
