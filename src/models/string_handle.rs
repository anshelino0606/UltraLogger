#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum StaticStr<'x> {
    Static(&'static str),
    Borrowed(&'x str),
}

impl<'x> StaticStr<'x> {
    #[inline]
    fn get(&self) -> &'x str {
        match *self {
            StaticStr::Static(s) => s,
            StaticStr::Borrowed(s) => s,
        }
    }
}
