#![warn(missing_docs)]
#![deny(missing_debug_implementations, unconditional_recursion)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]

use crate::models::constants::{
    DO_INIT_STATE, LOGGER_ERROR_SET, MAX_LOG_LEVEL_FILTER, PARSE_ERROR_LEVEL, STATE,
};
use crate::models::log::{Log, LOGGER};
use crate::models::{constants, level::LevelFilter};
use core::fmt;
use std::{mem, sync::atomic::Ordering};

pub(crate) fn is_ok<T, E>(t: Option<T>, e: E) -> Result<T, E> {
    match t {
        Some(t) => Ok(t),
        None => Err(e),
    }
}

#[inline]
#[cfg(target_has_atomic = "ptr")]
pub fn set_max_level(level: LevelFilter) {
    MAX_LOG_LEVEL_FILTER.store(level as usize, Ordering::Relaxed);
}

#[inline]
pub unsafe fn set_max_level_racy(level: LevelFilter) {
    MAX_LOG_LEVEL_FILTER.store(level as usize, Ordering::Relaxed);
}

#[inline(always)]
pub fn max_level() -> LevelFilter {
    unsafe { mem::transmute(MAX_LOG_LEVEL_FILTER.load(Ordering::Relaxed)) }
}

#[cfg(all(feature = "std", target_has_atomic = "ptr"))]
pub fn set_boxed_logger(logger: Box<dyn Log>) -> Result<(), SetLoggerError> {
    set_logger_inner(|| Box::leak(logger))
}

#[cfg(target_has_atomic = "ptr")]
pub fn set_logger(logger: &'static dyn Log) -> Result<(), SetLoggerError> {
    set_logger_inner(|| logger)
}

#[cfg(target_has_atomic = "ptr")]
fn set_logger_inner<F>(make_logger: F) -> Result<(), SetLoggerError>
where
    F: FnOnce() -> &'static dyn Log,
{
    let old_state = match STATE.compare_exchange(
        constants::NO_INIT_STATE,
        DO_INIT_STATE,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ) {
        Ok(s) | Err(s) => s,
    };
    match old_state {
        constants::NO_INIT_STATE => {
            unsafe {
                LOGGER = make_logger();
            }
            STATE.store(constants::INIT_STATE, Ordering::SeqCst);
            Ok(())
        }
        DO_INIT_STATE => {
            while STATE.load(Ordering::SeqCst) == DO_INIT_STATE {
                // TODO: replace with `hint::spin_loop` once MSRV is 1.49.0.
                #[allow(deprecated)]
                std::sync::atomic::spin_loop_hint();
            }
            Err(SetLoggerError(()))
        }
        _ => Err(SetLoggerError(())),
    }
}

pub unsafe fn set_logger_racy(logger: &'static dyn Log) -> Result<(), SetLoggerError> {
    match STATE.load(Ordering::SeqCst) {
        constants::NO_INIT_STATE => {
            LOGGER = logger;
            STATE.store(DO_INIT_STATE, Ordering::SeqCst);
            Ok(())
        }
        DO_INIT_STATE => {
            // This is just plain UB, since we were racing another initialization function
            unreachable!("set_logger_racy must not be used with other initialization functions")
        }
        _ => Err(SetLoggerError(())),
    }
}

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct SetLoggerError(());

impl fmt::Display for SetLoggerError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(LOGGER_ERROR_SET)
    }
}

#[cfg(feature = "std")]
impl error::Error for SetLoggerError {}

#[allow(missing_copy_implementations)]
#[derive(Debug, PartialEq, Eq)]
pub struct ParseLevelError(pub ());

impl ParseLevelError {
    pub fn new() -> Self {
        ParseLevelError(())
    }
}

impl fmt::Display for ParseLevelError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(PARSE_ERROR_LEVEL)
    }
}

#[cfg(feature = "std")]
impl error::Error for ParseLevelError {}

pub const STATIC_MAX_LEVEL: LevelFilter = MAX_LEVEL_INNER;

pub const MAX_LEVEL_INNER: LevelFilter = get_max_level_inner();

const fn get_max_level_inner() -> LevelFilter {
    #[allow(unreachable_code)]
    {
        #[cfg(all(not(debug_assertions), feature = "release_max_level_off"))]
        {
            return LevelFilter::Off;
        }
        #[cfg(all(not(debug_assertions), feature = "release_max_level_prod"))]
        {
            return LevelFilter::Prod;
        }
        #[cfg(all(not(debug_assertions), feature = "release_max_level_debug"))]
        {
            return LevelFilter::Debug;
        }
        #[cfg(all(not(debug_assertions), feature = "release_max_level_trace"))]
        {
            return LevelFilter::Trace;
        }
        #[cfg(feature = "max_level_off")]
        {
            return LevelFilter::Off;
        }
        #[cfg(feature = "max_level_prod")]
        {
            return LevelFilter::Prod;
        }
        #[cfg(feature = "max_level_debug")]
        {
            return LevelFilter::Debug;
        }
        LevelFilter::Trace
    }
}

impl Default for ParseLevelError {
    fn default() -> Self {
        Self::new()
    }
}
