pub mod constants;
pub mod level;
pub mod log;
pub mod metadata;
pub mod record;
pub mod string_handle;
pub(crate) mod threads;
mod utilities;

pub mod __private_api;
pub mod logger;
pub mod macros;


use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufReader, Write};
use std::io::Error as E;
use std::io::{BufRead, Read};
use std::os::raw::c_char;
use std::path::Path;
use std::time::Duration;
use crate::log;
use crate::models::constants::LOGGING_THREAD_TIMEOUT;
use crate::models::level::Level;
use crate::models::log::{Log, logger};
use crate::models::logger::{AsyncLogger, set_max_log_level};

#[repr(C)]
pub struct FfiResult {
    pub success: bool,
    pub error: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum LogLevel {
    None = 0,
    Prod = 1,
    Debug = 2,
    Trace = 3,
}

#[repr(C)]
pub struct FfiStringResult {
    pub result: *const c_char,
    pub error: *const c_char,
}

#[repr(C)]
pub struct LogData {
    args: *const c_char,
    level: LogLevel,
    source: *const c_char,
}

#[no_mangle]
pub extern "C" fn start_logging() {
    AsyncLogger::start_background_task();
}
#[no_mangle]
pub extern "C" fn log_message(log_data: LogData) {
    let args = unsafe { CString::from_raw(log_data.args as *mut _) }.to_string_lossy().into_owned();
    let source = unsafe { CString::from_raw(log_data.source as *mut _) }.to_string_lossy().into_owned();

    let log_level = match log_data.level {
        LogLevel::Trace => Level::Trace,
        LogLevel::Debug => Level::Debug,
        LogLevel::Prod => Level::Prod,
        LogLevel::None => Level::Prod
    };

    log!(source: &source.clone(), log_level, args.clone());
}

#[no_mangle]
pub extern "C" fn log_trace(source: *const c_char, message: *const c_char) {
    let string_source = unsafe { CStr::from_ptr(source) }.to_string_lossy().into_owned();
    let string_message = unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned();

    log!(source: &string_source.clone(), Level::Trace, &string_message.clone());
}

#[no_mangle]
pub extern "C" fn log_debug(source: *const c_char, message: *const c_char) {
    let string_source = unsafe { CStr::from_ptr(source) }.to_string_lossy().into_owned();
    let string_message = unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned();

    log!(source: &string_source.clone(), Level::Debug, &string_message.clone());
}

#[no_mangle]
pub extern "C" fn log_prod(source: *const c_char, message: *const c_char) {
    let string_source = unsafe { CStr::from_ptr(source) }.to_string_lossy().into_owned();
    let string_message = unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned();

    log!(source: &string_source.clone(), Level::Prod, &string_message.clone());
}

#[no_mangle]
pub extern "C" fn read_logs(file_path: *const c_char) -> FfiStringResult {
    let file_path_str = unsafe { CStr::from_ptr(file_path) }
        .to_str()
        .expect("ERROR::INVALID UTF8");
    let file_path = Path::new(file_path_str);

    let mut result = FfiStringResult {
        result: std::ptr::null(),
        error: std::ptr::null(),
    };

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        let logs: Vec<String> = reader
            .lines()
            .collect::<Result<_, E>>()
            .unwrap_or_else(|e| {
                result.error = CString::new(e.to_string())
                    .expect("ERROR::CSTRING CONVERSION FAILED")
                    .into_raw();
                Vec::new()
            });
        let logs_str = logs.join("\n");
        result.result = CString::new(logs_str)
            .expect("ERROR::CSTRING CONVERSION FAILED")
            .into_raw();
    } else {
        result.error = CString::new("ERROR::FAILED TO OPEN FILE")
            .expect("ERROR::CSTRING CONVERSION FAILED")
            .into_raw();
    }

    result
}

#[no_mangle]
pub extern "C" fn set_log_level(log_level: LogLevel) {
    let log_level = match log_level {
        LogLevel::Trace => Level::Trace,
        LogLevel::Debug => Level::Debug,
        LogLevel::Prod => Level::Prod,
        LogLevel::None => Level::Prod
    };

    set_max_log_level(log_level);
}


#[no_mangle]
pub extern "C" fn flush_logger() {
    logger().flush();
}

#[no_mangle]
pub extern "C" fn cleanup_logger() {
    std::thread::sleep(Duration::from_secs(LOGGING_THREAD_TIMEOUT));
}

