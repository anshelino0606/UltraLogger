#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

use crate::models::constants::{DATE_FORMAT, NUM_LOGGING_THREADS, TIME_FORMAT};
use crate::models::level::Level;
use crate::models::log::Log;
use crate::models::metadata::LogInfo;
use crate::models::record::LogRecord;
use crate::models::threads::{thread_id, ThreadPool};
use chrono::Utc;
use crossbeam_queue::SegQueue;
use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use crate::log_console;lazy_static::lazy_static! {
    static ref THREAD_POOL: ThreadPool = ThreadPool::new(NUM_LOGGING_THREADS);
    static ref MAX_LOG_LEVEL_FILTER: AtomicUsize = AtomicUsize::new(Level::Trace as usize);
    static ref LOG_QUEUE :  Arc<Mutex<SegQueue<LogData>>> = Arc::new(Mutex::new(SegQueue::new()));
}

#[derive(Clone)]
struct LogData {
    args: String,
    level: Level,
    source: String,
}

pub(crate) struct AsyncLogger;

impl<'x> Log for AsyncLogger {
    fn enabled(&self, metadata: &LogInfo) -> bool {
        metadata.level <= Level::from_usize(MAX_LOG_LEVEL_FILTER.load(Ordering::Relaxed)+1).unwrap()
    }

    fn log(&self, record: &LogRecord) {
        let log_data = LogData {
            args: record.args.clone(),
            level: record.metadata.level,
            source: record.metadata.source.to_string(),
        };
        if self.enabled(&record.metadata) {
            LOG_QUEUE.lock().unwrap().push(log_data.clone());
        }

    }

    fn flush(&self) {
        // TODO: Implement logic to flush logs, ensuring all logs are processed
    }
}

impl AsyncLogger {
    pub fn new() -> Self {
        AsyncLogger
    }

    pub fn start_background_task() {
        for _ in 0..NUM_LOGGING_THREADS {
            let log_queue = Arc::clone(&LOG_QUEUE);
            THREAD_POOL.execute(move || loop {
                if let Some(log_data) = log_queue.lock().unwrap().pop() {
                    let current_thread = thread_id();
                    log_console!(&log_data.source.clone(), log_data.level.clone(), current_thread.clone(), log_data.args.clone());
                    AsyncLogger::write_log_to_file(&log_data, current_thread);
                }
                thread::sleep(std::time::Duration::from_millis(100));
            });
        }
    }

    #[inline]
    fn write_log_to_file(log_record: &LogData, current_thread: u64) {
        let current_date = Utc::now();
        let formatted_date = current_date.format(DATE_FORMAT).to_string();
        let file_path_str = format!("{}_{:?}.log", formatted_date, log_record.level);

        if let Ok(file_path_cstr) = CString::new(file_path_str.clone()) {
            let file_path = file_path_cstr.to_string_lossy().to_string();
            if let Ok(file) = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(file_path)
            {
                let mut buf_writer = BufWriter::new(file);
                let log_entry = format!(
                    "[{:?}] - [{:<60}] - [{:<30}] - [{:<19}] - [{}]",
                    log_record.level,
                    log_record.source.trim(),
                    log_record.args.trim(),
                    formatted_date.trim(),
                    current_thread
                );

                if let Err(e) = writeln!(&mut buf_writer, "{}", log_entry) {
                    eprintln!("ERROR::FILE DIDN'T OPEN: {}", e);
                }
                if let Err(e) = buf_writer.flush() {
                    eprintln!("ERROR::FLUSHING BUFFER: {}", e);
                }
            } else {
                eprintln!("ERROR::FILE DIDN'T OPEN: {}", file_path_str);
            }
        } else {
            eprintln!("ERROR::CREATING CSTRING FROM FILE PATH: {}", file_path_str);
        }
    }
}

pub fn set_max_log_level(level: Level) {
    MAX_LOG_LEVEL_FILTER.store(level as usize, Ordering::Relaxed);
}
