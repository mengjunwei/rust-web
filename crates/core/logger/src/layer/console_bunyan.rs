//! 输出到控制台
//! 该层专门涉及使用Bunyan格式格式化信息。
//! 它依赖于上游的JsonStorageLayer来访问连接到每个跨度的字段。
use crate::config::ConsoleBunyanConfig;

use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, registry::LookupSpan, Layer,
};

/// 输出到控制台中
pub fn layer<S>(config: &ConsoleBunyanConfig) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: SubscriberExt,
    S: for<'a> LookupSpan<'a>,
{
    // Shared configuration regardless of where logs are output to.
    let layer = BunyanFormattingLayer::new(
        "console_bunyan_layer".into(),
        std::io::stdout.with_max_level(config.level.clone().into()),
    );
    Box::new(layer)
}