mod environment;
mod mysql;
mod server;
mod sqlite;

use code::Error;
use logger::config::Logger;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::sync::OnceLock;
use tracing::error;

static GLOBAL_CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// 环境配置
    #[serde(default)]
    pub environment: environment::Environment,
    /// 服务配置
    #[serde(default)]
    pub server: server::Server,
    /// Sqlite3 数据库配置
    #[serde(default)]
    pub sqlite: sqlite::Sqlite,
    /// Mysql 数据库配置
    #[serde(default)]
    pub mysql: mysql::Mysql,
    /// 日志配置
    #[serde(default)]
    pub logger: Logger,
}

pub fn init(path: &str) -> Result<AppConfig, Error> {
    let content = read_to_string(path)?;
    let config: AppConfig = serde_yaml::from_str(&content).map_err(|err| {
        error!("{}, err: {err}", Error::ConfigFileParseError);
        Error::ConfigFileParseError
    })?;
    GLOBAL_CONFIG.get_or_init(|| config.clone());
    Ok(config)
}
