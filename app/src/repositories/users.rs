use crate::repositories::DBPool;

pub trait UsersRepository {
    fn new() -> Self;
    fn get_user(&self, _db: DBPool) -> Result<(), ()>;
}

#[derive(Debug)]
pub struct Users;

impl UsersRepository for Users {
    fn new() -> Self {
        Self {}
    }
    fn get_user(&self, _db: DBPool) -> Result<(), ()> {
        Ok(())
    }
}
