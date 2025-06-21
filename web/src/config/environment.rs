use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Environment {
    pub env: String,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            env: "prod".to_string(),
        }
    }
}
