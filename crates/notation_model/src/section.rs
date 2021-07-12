use std::sync::Arc;

use crate::prelude::{Bar, SectionKind};

#[derive(Debug)]
pub struct Section {
    pub kind: SectionKind,
    pub bars: Vec<Arc<Bar>>,
}
impl From<(SectionKind, Vec<Arc<Bar>>)> for Section {
    fn from(v: (SectionKind, Vec<Arc<Bar>>)) -> Self {
        Self {
            kind: v.0,
            bars: v.1,
        }
    }
}
