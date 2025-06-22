pub mod config;
pub mod layer;
pub mod utils;

use tracing::subscriber::SetGlobalDefaultError;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::{Layer, Registry};

#[derive(Debug)]
pub enum Error {
    ColorEyreReport(color_eyre::Report),
    SetGlobalDefaultError(SetGlobalDefaultError),
}

type RegistryLayer = Box<dyn Layer<Layered<ErrorLayer<Registry>, Registry>> + Send + Sync>;

struct LoggerLayer<'a> {
    layers: Vec<RegistryLayer>,
    guards: Vec<WorkerGuard>,
    config: &'a config::Logger,
}

impl<'a> LoggerLayer<'a> {
    /// 从配置创建对象
    fn form_config(config: &'a config::Logger) -> Self {
        let layers = Vec::new();
        let guards = Vec::new();
        LoggerLayer {
            layers,
            guards,
            config,
        }
    }

    /// 输出到控制台中
    fn set_console(&mut self) -> &mut Self {
        if !self.config.console.enable {
            return self;
        }

        let layer = layer::console::layer(&self.config.console);
        self.layers.push(layer);
        self
    }

    /// bunyan 日志输出到控制台中
    fn set_console_bunyan(&mut self) -> &mut Self {
        if !self.config.console_bunyan.enable {
            return self;
        }

        let layer = layer::console_bunyan::layer(&self.config.console_bunyan);
        self.layers.push(layer);
        self
    }

    /// 输出到文件中
    fn set_file(&mut self) -> &mut Self {
        if !self.config.file.enable {
            return self;
        }

        let (file_layer, file_guard) = layer::file::non_blocking_layer(&self.config.file);
        self.layers.push(file_layer);
        self.guards.push(file_guard);
        self
    }

    /// 输出到数据库
    fn set_db(&mut self) -> &mut Self {
        if !self.config.db.enable {
            return self;
        }

        // let (layer, guard) = layer::db::non_blocking_layer(self.config.db.clone());
        // self.layers.push(layer);
        // self.guards.push(guard);
        self
    }

    /// 构建对象
    pub fn build(config: &'a config::Logger) -> (Vec<RegistryLayer>, Vec<WorkerGuard>) {
        let mut binding = Self::form_config(config);
        let layer = binding
            .set_console()
            .set_console_bunyan()
            .set_file()
            .set_db();

        (
            std::mem::take(&mut layer.layers),
            std::mem::take(&mut layer.guards),
        )
    }
}

pub struct Logger<'a> {
    config: &'a config::Logger,
}

impl<'a> Logger<'a> {
    fn from_config(config: &'a config::Logger) -> Self {
        Logger { config }
    }

    fn set_color_eyre(&mut self) -> Result<&mut Self, Error> {
        if !self.config.color_eyre {
            return Ok(self);
        }
        color_eyre::install().map_err(Error::ColorEyreReport)?;
        Ok(self)
    }

    fn set_global_default(&mut self, layers: Vec<RegistryLayer>) -> Result<(), Error> {
        // 日志订阅器
        let subscriber = Registry::default()
            // .with(level_filter)
            // ErrorLayer 可以让 color-eyre 获取到 span 的信息
            .with(ErrorLayer::default())
            // .with(console_layer)
            .with(layers);

        // 注册全局日志订阅器
        tracing::subscriber::set_global_default(subscriber).map_err(Error::SetGlobalDefaultError)
    }

    pub fn build(config: &'a config::Logger) -> Result<Vec<WorkerGuard>, Error> {
        let (layers, guards) = LoggerLayer::build(config);
        Self::from_config(config)
            .set_color_eyre()?
            .set_global_default(layers)?;
        Ok(guards)
    }
}
