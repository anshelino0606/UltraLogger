#![warn(missing_docs)]
#![deny(missing_debug_implementations, unconditional_recursion)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]
#[cfg(not(target_has_atomic = "ptr"))]
use std::cell::Cell;
use std::sync::atomic::AtomicUsize;

pub const DATE_FORMAT: &str =                "%Y-%m-%d";
pub const TIME_FORMAT: &str =                "%Y-%m-%d %H:%M:%S";
pub const LOGGING_THREAD_TIMEOUT: u64 =      2; // in seconds; don't mess with it unless you know what you're doing
pub const NUM_LOGGING_THREADS: usize =       4;

pub const NO_INIT_STATE: usize =             0;
pub const DO_INIT_STATE: usize =             1;
pub const INIT_STATE: usize =                2;

#[cfg(not(target_has_atomic = "ptr"))]
struct AtomicUsize {
    v: Cell<usize>,
}

#[cfg(not(target_has_atomic = "ptr"))]
impl AtomicUsize {
    const fn new(v: usize) -> AtomicUsize {
        AtomicUsize { v: Cell::new(v) }
    }

    fn load(&self, _order: Ordering) -> usize {
        self.v.get()
    }

    fn store(&self, val: usize, _order: Ordering) {
        self.v.set(val)
    }

    #[cfg(target_has_atomic = "ptr")]
    fn compare_exchange(
        &self,
        current: usize,
        new: usize,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<usize, usize> {
        let prev = self.v.get();
        if current == prev {
            self.v.set(new);
        }
        Ok(prev)
    }
}

#[cfg(not(target_has_atomic = "ptr"))]
unsafe impl Sync for AtomicUsize {}

#[no_mangle]
pub(crate) static STATE: AtomicUsize = AtomicUsize::new(2);

#[no_mangle]
pub(crate) static MAX_LOG_LEVEL_FILTER: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub(crate) static LOG_LEVEL_NAMES: [&str; 4] = ["None", "Prod", "Debug", "Trace"];

#[no_mangle]
pub(crate) static LOGGER_ERROR_SET: &str =
    "ERROR::SETTING LOGGER AFTER LOGGING SYSTEM WAS INITIALIZED";

#[no_mangle]
pub(crate) static PARSE_ERROR_LEVEL: &str =
    "ERROR::PARSING STRING ERROR::DOESN'T MATCH ANY LOG LEVEL";
