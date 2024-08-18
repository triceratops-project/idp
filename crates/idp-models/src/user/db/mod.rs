mod active_model;
mod column;
mod model;
mod primary_key;
mod relation;

use std::fmt;

pub use active_model::ActiveModel;
pub use column::Column;
pub use model::Model;
pub use primary_key::PrimaryKey;
pub use relation::Relation;

use sea_orm::{EntityName, EntityTrait, Iden, IdenStatic};

#[derive(Copy, Clone, Debug, Default)]
pub struct Entity;

impl EntityTrait for Entity {
    type Model = Model;
    type Column = Column;
    type PrimaryKey = PrimaryKey;
    type ActiveModel = ActiveModel;
    type Relation = Relation;
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users"
    }
}

impl IdenStatic for Entity {
    fn as_str(&self) -> &str {
        Self::table_name(self)
    }
}

impl Iden for Entity {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        s.write_fmt(format_args!("{0}", IdenStatic::as_str(self)))
            .unwrap();
    }
}
