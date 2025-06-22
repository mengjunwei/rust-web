mod config;
mod server;
mod router;

use app_state::AppState;
use clap::{Arg, Command};
use inject::InjectProvider;
use std::fmt::Write;
use std::sync::Arc;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let m = Command::new("rule engine")
        .author("Mengjunwei")
        .version("1.0.0")
        .about("Explains in brief what the program does")
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .help("config path")
                .default_value("config.yaml")
                .default_missing_value("config.yaml"),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        );
    let conf_arg = m.get_matches();
    let conf = conf_arg.get_one::<String>("config").unwrap();
    let mut real_conf_file = String::new();
    if let Ok(val) = dotenv::var("RUST_WEB_CONFIG") {
        real_conf_file
            .write_str(&val)
            .expect("error writing REAL_CONF_FILE");
    } else {
        real_conf_file
            .write_str(&conf)
            .expect("error writing REAL_CONF_FILE");
    }

    // 加载配置文件
    let conf = match config::init(&real_conf_file) {
        Ok(v) => v,
        Err(err) => {
            panic!("配置文件加载失败, err: {err}")
        }
    };
    let _guards = logger::Logger::build(&conf.logger).expect("初始化日志失败");
    info!("{:?}", conf);

    let database_url = conf.mysql.write.dns();
    let db_pool = database::Pool::new(database_url, conf.mysql.options.clone())
        .await
        .expect("初始化数据库失败");

    let provider = InjectProvider::new(Arc::new(db_pool.clone()));
    let provider = Arc::new(provider);
    // 启动服务, 并阻塞

    // 共享状态
    let app_state = AppState {};
    if let Err(e) = server::start(app_state, provider, conf).await {
        panic!("server start faild. err: {e}");
    }
    info!("close service...");
    Ok(())
}
