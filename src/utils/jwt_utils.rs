use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
}

pub struct JwtUtils {
    secret: String,
}

impl JwtUtils {
    pub fn new(secret: String) -> Self {
        JwtUtils { secret }
    }

    pub fn generate_token(&self, user_id: &str, duration: Duration) -> Result<String, String> {
        let expiration_time = SystemTime::now()
            .checked_add(duration)
            .ok_or("Failed to calculate expiration time")?;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration_time.duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_secs(),
        };

        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(self.secret.as_bytes());
        encode(&header, &claims, &encoding_key).map_err(|e| e.to_string())
    }

    pub fn validate_token(&self, token: &str) -> Result<String, String> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_bytes());

        decode::<Claims>(token, &decoding_key, &Validation::default())
            .map(|data| data.claims.sub)
            .map_err(|e| e.to_string())
    }
}