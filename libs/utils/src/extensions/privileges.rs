use crate::auth::privileges::Privilege;

#[derive(Clone)]
pub struct RequiredPrivileges(pub Vec<Privilege>);
