use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier, password_hash::Salt};

pub fn hash_password(password: &str, salt: Salt) -> Result<String, argon2::password_hash::Error> {
    // Set up the Argon2 configuration
    // let config = Config::default();
    let argon2 = Argon2::default();

    // Hash the password
    let hash = argon2.hash_password(password.as_bytes(), salt)?;

    // Convert the hash to bytes
    Ok(hash.to_string())
}

/// Verifies a plaintext password against a hashed password.
///
/// # Arguments
/// * `password` - The plaintext password to verify.
/// * `hashed_password` - The hashed password to compare against.
///
/// # Returns
/// * `Ok(bool)` - `true` if the password matches, `false` otherwise.
/// * `Err(Argon2Error)` - If verification fails.
pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, argon2::password_hash::Error> {
    // Parse the hashed password string into a `PasswordHash`
    let parsed_hash = PasswordHash::new(hashed_password)?;

    // Create an instance of Argon2
    let argon2 = Argon2::default();

    // Verify the password against the hashed password
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}