#![warn(missing_docs)]
#![deny(unconditional_recursion)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]

use crate::models::logger::AsyncLogger;
use crate::models::{
    constants::NO_INIT_STATE, constants::STATE, metadata::LogInfo, record::LogRecord,
};
use std::sync::atomic::Ordering;

#[no_mangle]
pub(crate) static mut LOGGER: &'static dyn Log = &AsyncLogger;

pub trait Log: Sync + Send {
    fn enabled(&self, metadata: &LogInfo) -> bool;
    fn log(&self, record: &LogRecord);
    fn flush(&self);
}

pub struct NopLogger;

impl Log for NopLogger {
    fn enabled(&self, _: &LogInfo) -> bool {
        false
    }

    fn log(&self, _: &LogRecord) {}
    fn flush(&self) {}
}

impl<T> Log for &'_ T
where
    T: ?Sized + Log,
{
    fn enabled(&self, metadata: &LogInfo) -> bool {
        (**self).enabled(metadata)
    }

    fn log(&self, record: &LogRecord) {
        (**self).log(record);
    }
    fn flush(&self) {
        (**self).flush();
    }
}

#[cfg(feature = "std")]
impl<T> Log for std::boxed::Box<T>
where
    T: ?Sized + Log,
{
    fn enabled(&self, metadata: &LogInfo) -> bool {
        self.as_ref().enabled(metadata)
    }

    fn log(&self, record: &LogRecord) {
        self.as_ref().log(record)
    }
    fn flush(&self) {
        self.as_ref().flush()
    }
}

#[cfg(feature = "std")]
impl<T> Log for std::sync::Arc<T>
where
    T: ?Sized + Log,
{
    fn enabled(&self, metadata: &LogInfo) -> bool {
        self.as_ref().enabled(metadata)
    }

    fn log(&self, record: &LogRecord) {
        self.as_ref().log(record)
    }
    fn flush(&self) {
        self.as_ref().flush()
    }
}

pub fn logger() -> &'static dyn Log {
    if STATE.load(Ordering::SeqCst) == NO_INIT_STATE {
        static NOP: NopLogger = NopLogger;
        &NOP
    } else {
        unsafe { LOGGER }
    }
}
