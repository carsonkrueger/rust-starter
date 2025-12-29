use std::str::FromStr;

use models::db::auth::privilege;
use strum::{EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoStaticStr, EnumString, EnumIter)]
pub enum Privilege {
    UserRead,
    UserWrite,
    UserDelete,
    UserCreate,

    UsersRead,
    UsersWrite,
    UsersDelete,
    UsersCreate,
}

impl TryFrom<privilege::Privilege> for Privilege {
    type Error = strum::ParseError;

    fn try_from(value: privilege::Privilege) -> Result<Self, Self::Error> {
        Self::from_str(&value.name)
    }
}
