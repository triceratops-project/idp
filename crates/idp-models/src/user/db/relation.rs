use sea_orm::{EnumIter, RelationTrait};

#[derive(Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        match self {
            _ => unreachable!("users table has no relations"),
        }
    }
}
