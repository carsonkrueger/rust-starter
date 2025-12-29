use argon2::{
    PasswordHash, PasswordVerifier,
    password_hash::{self, PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash_password<'a>(password: &str) -> password_hash::Result<String> {
    let argon = argon2::Argon2::default();
    let salt = &SaltString::generate(&mut OsRng);
    let hash = argon.hash_password(password.as_bytes(), salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> password_hash::Result<()> {
    let argon = argon2::Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;
    argon.verify_password(password.as_bytes(), &parsed_hash)
}
