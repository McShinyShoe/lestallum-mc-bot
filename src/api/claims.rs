use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(owner: String) -> Self {
        Self {
            sub: owner,
            exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
        }
    }
}