use orion::pwhash::{self, PasswordHash};
use serde::{Deserialize, Serialize};
use slink_common::{ApiError, ApiResult, HASHING_ITERATIONS, HASHING_MEMORY};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new(password: impl Into<String>) -> ApiResult<Self> {
        let pw = orion::pwhash::Password::from_slice(Into::<String>::into(password).as_bytes())
            .or_else(|e| Err(ApiError::CryptographicError(e.to_string())))?;
        let hashed = pwhash::hash_password(&pw, HASHING_ITERATIONS, HASHING_MEMORY)
            .or_else(|e| Err(ApiError::CryptographicError(e.to_string())))?;
        Ok(Self(hashed.unprotected_as_encoded().to_string()))
    }

    fn hash(&self) -> PasswordHash {
        PasswordHash::from_encoded(&self.0).expect("Unable to decode encoded password hash.")
    }

    pub fn verify(&self, password: impl Into<String>) -> bool {
        if let Ok(parsed) =
            orion::pwhash::Password::from_slice(Into::<String>::into(password).as_bytes())
        {
            pwhash::hash_password_verify(&self.hash(), &parsed).is_ok()
        } else {
            false
        }
    }
}

impl Into<String> for HashedPassword {
    fn into(self) -> String {
        self.0.clone()
    }
}
