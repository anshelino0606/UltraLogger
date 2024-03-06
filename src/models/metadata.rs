#![warn(missing_docs)]
#![deny(missing_debug_implementations, unconditional_recursion)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]
use crate::models::level::Level;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LogInfo<'x> {
    pub(crate) level: Level,
    pub(crate) source: &'x str,
}

impl<'x> LogInfo<'x> {
    #[inline]
    pub fn builder() -> LogInfoBuilder<'x> {
        LogInfoBuilder::new()
    }

    #[inline]
    pub fn level(&self) -> Level {
        self.level
    }
    #[inline]
    pub fn source(&self) -> &'x str {
        self.source
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LogInfoBuilder<'x> {
    metadata: LogInfo<'x>,
}

impl<'x> LogInfoBuilder<'x> {
    #[inline]
    pub fn new() -> LogInfoBuilder<'x> {
        LogInfoBuilder {
            metadata: LogInfo {
                level: Level::Prod,
                source: "",
            },
        }
    }

    #[inline]
    pub fn level(&mut self, arg: Level) -> &mut LogInfoBuilder<'x> {
        self.metadata.level = arg;
        self
    }

    #[inline]
    pub fn source(&mut self, source: &'x str) -> &mut LogInfoBuilder<'x> {
        self.metadata.source = source;
        self
    }

    #[inline]
    pub fn build(&self) -> LogInfo<'x> {
        self.metadata.clone()
    }
}

impl<'x> Default for LogInfoBuilder<'x> {
    fn default() -> Self {
        Self::new()
    }
}
