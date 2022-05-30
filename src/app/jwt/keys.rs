use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Clone)]
pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret_bytes: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret_bytes),
            decoding: DecodingKey::from_secret(secret_bytes),
        }
    }

    pub fn get_encoding_key(&self) -> &EncodingKey {
        &self.encoding
    }

    pub fn get_decoding_key(&self) -> &DecodingKey {
        &self.decoding
    }
}
