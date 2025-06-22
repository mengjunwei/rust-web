//! 输出到文件
#![allow(unused)]

use crate::config::FileConfig;
use crate::utils::time::local_time;

use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::Layer;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::registry::LookupSpan;

pub fn non_blocking_layer<S>(
    config: &FileConfig,
) -> (Box<dyn Layer<S> + Send + Sync + 'static>, WorkerGuard)
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    // 本地时间
    let timer = local_time();

    // Shared configuration regardless of where logs are output to.
    let file_appender = rolling::daily(config.filepath.clone(), config.filename.clone());

    // 非阻塞
    let (non_blocking, guard) = non_blocking(file_appender);

    let layer = fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_timer(timer)
        .with_writer(non_blocking.with_max_level(config.level.clone().into()))
        .boxed();
    (layer, guard)
}
