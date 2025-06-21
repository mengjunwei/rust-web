use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub base: Base,
    pub captcha: Captcha,
    pub upload: Upload,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            base: Base::default(),
            captcha: Captcha::default(),
            upload: Upload::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Base {
    pub address: String,
    pub port: u32,
}

impl Base {
    pub fn address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

impl Default for Base {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 8000,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Captcha {
    pub expire: i8,
}

impl Default for Captcha {
    fn default() -> Self {
        Self { expire: 60 }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Upload {
    pub filepath: String,
}

impl Default for Upload {
    fn default() -> Self {
        Self {
            filepath: "./upload".to_string(),
        }
    }
}
