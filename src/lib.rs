pub mod models;

use crate::models::level::Level;
use crate::models::log::Log;
use crate::models::logger::AsyncLogger;
use crate::models::metadata::LogInfo;
use crate::models::record::LogRecordBuilder;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize the async logger and the thread pool
    let async_logger = AsyncLogger::new();
    AsyncLogger::start_background_task();

    // This Arc allows multiple threads to hold a reference to the logger
    let logger = Arc::new(async_logger);

    // Simulate multiple clients logging messages
    let client_count = 10;
    let mut handles = vec![];

    for i in 0..client_count {

        let handle = thread::spawn(move || {

            // Simulate some work being done
            thread::sleep(Duration::from_millis(10));
        });

        handles.push(handle);
    }

    let logger_clone = Arc::clone(&logger);

    let handle = thread::spawn(move || {
        let log_info = LogInfo::builder()
            .level(Level::Trace)
            .source("Main")
            .build();
        let message = "Log message from main client".to_string();
        let log_record = LogRecordBuilder::new()
            .args(message)
            .metadata(log_info)
            .build();

        // Log the message
        logger_clone.log(&log_record);

        // Simulate some work being done
        thread::sleep(Duration::from_millis(10));
    });

    handles.push(handle);

    // Wait for all simulated clients to finish logging


    // Optionally, sleep for a bit to allow the background logging tasks to process all messages
    thread::sleep(Duration::from_secs(10));

    for handle in handles {
        handle.join().unwrap();
    }


}
