#![warn(missing_docs)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]

use crate::models::{level::Level, metadata::LogInfo, string_handle::StaticStr};
use core::fmt;

#[cfg(feature = "kv_unstable")]
#[derive(Clone)]
struct KeyValues<'x>(&'x dyn kv::Source);

#[derive(Clone, Debug)]
pub struct LogRecord<'x> {
    pub(crate) metadata: LogInfo<'x>,
    pub(crate) args: String,
    pub(crate) module_path: Option<StaticStr<'x>>,
    pub(crate) file: Option<StaticStr<'x>>,
    pub(crate) line: Option<u32>,
    #[cfg(feature = "kv_unstable")]
    key_values: KeyValues<'x>,
}

#[derive(Debug)]
pub struct LogRecordBuilder<'x> {
    record: LogRecord<'x>,
}

#[cfg(feature = "kv_unstable")]
impl<'x> fmt::Debug for KeyValues<'x> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut visitor = f.debug_map();
        self.0.visit(&mut visitor).map_err(|_| fmt::Error)?;
        visitor.finish()
    }
}

impl<'x> LogRecordBuilder<'x> {
    #[inline]
    pub fn new() -> LogRecordBuilder<'x> {
        LogRecordBuilder {
            record: LogRecord {
                args: "".to_string(),
                metadata: LogInfo::builder().build(),
                module_path: None,
                file: None,
                line: None,
                #[cfg(feature = "kv_unstable")]
                key_values: KeyValues(&Option::None::<(kv::Key, kv::Value)>),
            },
        }
    }

    #[inline]
    pub fn args(&mut self, args: String) -> &mut LogRecordBuilder<'x> {
        self.record.args = args;
        self
    }

    #[inline]
    pub fn metadata(&mut self, metadata: LogInfo<'x>) -> &mut LogRecordBuilder<'x> {
        self.record.metadata = metadata;
        self
    }

    #[inline]
    pub fn level(&mut self, level: Level) -> &mut LogRecordBuilder<'x> {
        self.record.metadata.level = level;
        self
    }

    #[inline]
    pub fn source(&mut self, source: &'x str) -> &mut LogRecordBuilder<'x> {
        self.record.metadata.source = source;
        self
    }

    #[inline]
    pub fn module_path(&mut self, path: Option<&'x str>) -> &mut LogRecordBuilder<'x> {
        self.record.module_path = path.map(StaticStr::Borrowed);
        self
    }

    #[inline]
    pub fn module_path_static(&mut self, path: Option<&'static str>) -> &mut LogRecordBuilder<'x> {
        self.record.module_path = path.map(StaticStr::Static);
        self
    }

    #[inline]
    pub fn file(&mut self, file: Option<&'x str>) -> &mut LogRecordBuilder<'x> {
        self.record.file = file.map(StaticStr::Borrowed);
        self
    }

    #[inline]
    pub fn file_static(&mut self, file: Option<&'static str>) -> &mut LogRecordBuilder<'x> {
        self.record.file = file.map(StaticStr::Static);
        self
    }

    #[inline]
    pub fn line(&mut self, line: Option<u32>) -> &mut LogRecordBuilder<'x> {
        self.record.line = line;
        self
    }

    #[cfg(feature = "kv_unstable")]
    #[inline]
    pub fn key_values(&mut self, kvs: &'x dyn kv::Source) -> &mut LogRecordBuilder<'x> {
        self.record.key_values = KeyValues(kvs);
        self
    }

    #[inline]
    pub fn build(&self) -> LogRecord<'x> {
        self.record.clone()
    }
}

impl<'x> Default for LogRecordBuilder<'x> {
    fn default() -> Self {
        Self::new()
    }
}
