//! Password hashing and verification using Argon2.

use crate::{Error, Result};
use argon2::Config;
use rand::Rng;

pub fn hash(password: &str) -> Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|_e| Error::internal_error())
}

pub fn verify(hash: &str, attempted_password: &str) -> Result<()> {
    let verified = argon2::verify_encoded(hash, attempted_password.as_bytes())
        .map_err(|_e| Error::internal_error())?;

    if !verified {
        Err(Error::identity_invalid())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify() {
        let password = "some_password";
        let hashed_password = hash(password).unwrap();
        let result = verify(&hashed_password, password);

        assert_eq!(result, Ok(()));
    }
}
