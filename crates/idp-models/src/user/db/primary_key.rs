use std::fmt;

use super::Column;
use sea_orm::{EnumIter, Iden, IdenStatic, PrimaryKeyToColumn, PrimaryKeyTrait};

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;

    fn auto_increment() -> bool {
        false
    }
}

impl PrimaryKeyToColumn for PrimaryKey {
    type Column = Column;

    fn into_column(self) -> Self::Column {
        match self {
            Self::Id => Self::Column::Id,
        }
    }

    fn from_column(col: Self::Column) -> Option<Self> {
        match col {
            Self::Column::Id => Some(Self::Id),
            _ => None,
        }
    }
}

impl IdenStatic for PrimaryKey {
    fn as_str(&self) -> &str {
        match self {
            Self::Id => "id",
        }
    }
}

impl Iden for PrimaryKey {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        s.write_fmt(format_args!("{0}", IdenStatic::as_str(self)))
            .unwrap();
    }
}
