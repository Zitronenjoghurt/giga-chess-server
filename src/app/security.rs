use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn hash_bytes(bytes: &[u8]) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2.hash_password(bytes, &salt)?;
    Ok(hash.to_string())
}

pub fn verify_bytes(bytes: &[u8], hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(bytes, &parsed_hash)
        .is_ok())
}
