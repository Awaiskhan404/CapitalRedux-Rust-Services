use jsonwebtoken::{decode, encode, Header, Validation};
use chrono::{Utc, Duration};

pub struct JwtService {
    secret_key: String,
}

impl JwtService {
    pub fn new(secret_key: String) -> Self {
        Self {
            secret_key,
        }
    }

    pub fn generate_token(&self, user_id: i32) -> Result<String, String> {
        let payload = Claims {
            sub: user_id.to_string(),
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
        };

        match encode(&Header::default(), &payload, self.secret_key.as_bytes()) {
            Ok(token) => Ok(token),
            Err(e) => Err(format!("Failed to generate token: {}", e)),
        }
    }

    pub fn verify_token(&self, token: &str) -> Result<i32, String> {
        match decode::<Claims>(&token, self.secret_key.as_bytes(), &Validation::default()) {
            Ok(decoded) => {
                let user_id = decoded.claims.sub.parse::<i32>().unwrap();
                Ok(user_id)
            }
            Err(e) => Err(format!("Failed to verify token: {}", e)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
