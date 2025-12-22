use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignUp {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

pub struct Login {
    pub email: String,
    pub password: String,
}
