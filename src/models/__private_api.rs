use crate::models::level::Level;
use crate::models::log::logger;
use crate::models::metadata::LogInfo;
use crate::models::record::LogRecordBuilder;
pub use std::option::Option;
pub use std::{file, format_args, line, module_path, stringify};
use chrono::Utc;
use crate::models::constants::{DATE_FORMAT, TIME_FORMAT};
use colored::Colorize;

#[cfg(not(feature = "kv_unstable"))]
pub fn log(
    args: String,
    level: Level,
    &(source, module_path, file): &(&str, &'static str, &'static str),
    line: u32,
    kvs: Option<&[(&str, &str)]>,
) {
    if kvs.is_some() {
        panic!("ERROR OF KVS")
    }

    logger().log(
        &LogRecordBuilder::new()
            .args(args.clone())
            .level(level.clone())
            .source(source.clone())
            .module_path_static(Some(module_path))
            .file_static(Some(file))
            .line(Some(line))
            .build(),
    );
}


#[cfg(not(feature = "kv_unstable"))]
pub fn log_console(
    args: String,
    level: Level,
    source: String,
    thread: u64,
    kvs: Option<&[(&str, &str)]>,
) {
    if kvs.is_some() {
        panic!("ERROR OF KVS")
    }

    let current_date = Utc::now();
    let formatted_date = current_date.format(TIME_FORMAT).to_string();

    let level_color = match level {
        Level::Prod => "green",
        Level::Debug => "cyan",
        Level::Trace => "magenta",
    };

    let log_entry = format!(
        "[{:<5}] - [{:<65}] - [{:<35}] - [{:<19}] - [{}]",
        format!("{:?}", level).color(level_color),
        source.color("blue"),
        args.color("bright_white"),
        formatted_date.color("yellow"),
        format!("{}", thread).color("bright_white")
    );

    println!("{}", log_entry);
}


#[cfg(feature = "kv_unstable")]
pub fn log(
    args: Arguments,
    level: Level,
    &(source, module_path, file): &(&str, &'static str, &'static str),
    line: u32,
    kvs: Option<&[(&str, &dyn crate::kv::ToValue)]>,
) {
    logger().log(
        &LogRecordBuilder::new()
            .args(args)
            .level(level)
            .source(source)
            .module_path_static(Some(module_path))
            .file_static(Some(file))
            .line(Some(line))
            .key_values(&kvs)
            .build(),
    );
}

pub fn log_enabled(level: Level, source: &str) -> bool {
    logger().enabled(&LogInfo::builder().level(level).source(source).build())
}
