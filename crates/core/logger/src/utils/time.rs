//! 时间工具

use time::{UtcOffset, format_description::FormatItem};
use tracing_subscriber::fmt::time::OffsetTime;

/// 获取本地时间
pub fn local_time() -> OffsetTime<Vec<FormatItem<'static>>> {
    let time_format = time::format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
    )
    .expect("format string should be valid!");

    let offset = UtcOffset::from_hms(8, 0, 0).unwrap();

    // 本地时间
    OffsetTime::new(offset, time_format)
}
