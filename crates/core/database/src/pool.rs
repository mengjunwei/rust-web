//! 数据库连接池
use std::{future::Future, pin::Pin, time::Duration};

use crate::config::DbOptions;

use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr,
};

/// 数据库特征
pub trait PoolTrait: Send + Sync {
    /// 获取数据库实例
    fn db(&self) -> &DatabaseConnection;

    /// 关闭数据库实例
    fn close(self) -> Pin<Box<dyn Future<Output = Result<(), DbErr>> + Send>>;
}

/// 数据库连接池
#[derive(Debug, Clone)]
pub struct Pool {
    /// DB 实例
    pub db: DatabaseConnection,
}

impl Pool {
    /// 初始化数据库连接池
    pub async fn new(db_url: String, options: DbOptions) -> Result<Pool, DbErr> {
        let db = Self::connect(db_url, options.clone()).await?;
        let pool = Pool { db };
        Ok(pool)
    }

    /// ## 连接数据库
    ///
    /// 数据库连接参数:
    /// - min_connections: 设置连接池的最小连接数。这是连接池会保持打开的最小数据库连接数量，即使这些连接当前没有被使用。
    /// - connect_timeout: 设置连接数据库时的超时时间（以秒为单位）。如果在这段时间内无法建立连接，操作将被取消并返回错误。
    /// - acquire_timeout: 设置从连接池获取连接时的超时时间（以秒为单位）。如果在这段时间内无法获取连接，操作将被取消并返回错误。
    /// - idle_timeout: 设置连接在被回max_connections: 设置连接池的最大连接数。这是连接池可以打开的最大数据库连接数量。当达到这个数量时，新的连接收之前可以保持空闲状态的最长时间（以秒为单位）。如果连接在这段时间内没有被使用，它将被关闭并从连接池中移除。
    /// - max_lifetime: 设置连接的最大生命周期（以秒为单位）。即使连接仍在使用中，超过这个时间后，它也会被关闭并从连接池中移除。
    /// - sqlx_logging: 启用或禁用 SQLx 日志记录。SQLx 是 SeaORM 底层使用的数据库驱动，这个选项控制是否记录 SQLx 的日志信息。
    /// - sqlx_logging_level: 设置 SQLx 日志记录的级别。这个选项决定了记录哪些级别的日志信息，例如错误、警告、信息或调试信息。
    ///
    /// 查看最大连接数：
    /// ```sql
    /// SHOW VARIABLES LIKE 'max_connections';
    /// ```
    pub async fn connect(db_url: String, options: DbOptions) -> Result<DatabaseConnection, DbErr> {
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(options.max_connections)
            .min_connections(options.min_connections)
            .connect_timeout(Duration::from_secs(options.connect_timeout))
            .acquire_timeout(Duration::from_secs(options.acquire_timeout))
            .idle_timeout(Duration::from_secs(options.idle_timeout))
            .max_lifetime(Duration::from_secs(options.max_lifetime))
            .sqlx_logging(options.logging_enable)
            .sqlx_logging_level(options.logging_level.into());
        let db = Database::connect(opt).await?;

        // 检查连接是否有效
        db.ping().await?;

        // 设置 Time Zone
        Self::set_time_zone(&db).await?;

        Ok(db)
    }

    /// 从连接生成连接池
    pub fn form_connect(db: DatabaseConnection) -> Pool {
        Pool { db }
    }

    /// 设置 Time Zone
    /// 不支持SQLite3
    #[allow(unused)]
    async fn set_time_zone(db: &DatabaseConnection) -> Result<(), DbErr> {
        if db.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }
        let stmt = sea_orm::Statement::from_string(
            db.get_database_backend(),
            "SET time_zone = '+08:00'".to_owned(),
        );
        db.execute(stmt).await?;
        Ok(())
    }
}

impl PoolTrait for Pool {
    /// 获取数据库实例
    fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    /// 关闭数据库实例
    fn close(self) -> Pin<Box<dyn Future<Output = Result<(), DbErr>> + Send>> {
        Box::pin(async move { self.db.close().await })
    }
}
