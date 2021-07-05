use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::prelude::Bar;

// https://www.masterclass.com/articles/songwriting-101-learn-common-song-structures
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum SectionKind {
    Intro,
    Verse,
    Chorus,
    Bridge,
    Outro,
    PreChorus,
    Solo,
}

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
