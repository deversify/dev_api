//! Password hashing and verification using Argon2.
//! # Example
//! ```
//! fn hash_and_verify() {
//!     let password = "some_password";
//!     let hashed_password = dev_api::password::hash(password).unwrap();
//!     let result = dev_api::password::verify(&hashed_password, password);
//!
//!     assert_eq!(result, Ok(()));
//! }
//! ```

use crate::{Error, Result};
use argon2::Config;
use rand::Rng;

pub type Hash = fn(password: &str) -> Result<String>;

pub fn hash(password: &str) -> Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|_e| Error::internal_error())
}

pub type Verify = fn(hash: &str, attempted_password: &str) -> Result<()>;

pub fn verify(hash: &str, attempted_password: &str) -> Result<()> {
    let verified = argon2::verify_encoded(hash, attempted_password.as_bytes())
        .map_err(|_e| Error::internal_error())?;

    if !verified {
        Err(Error::identity_invalid())
    } else {
        Ok(())
    }
}
