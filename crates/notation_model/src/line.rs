use fehler::{throw, throws};
use std::convert::TryFrom;
use std::fmt::Display;
use std::iter::Iterator;
use std::sync::Arc;

use crate::prelude::{ParseError, ProtoEntry};

#[derive(Debug)]
pub struct Line {
    pub key: String,
    pub entries: Vec<Arc<ProtoEntry>>,
}
#[derive(Debug)]
pub struct Slice {
    pub line: Arc<Line>,
    pub index: usize,
    pub count: usize,
}
impl Line {
    pub fn new(key: String, entries: Vec<Arc<ProtoEntry>>) -> Self {
        Self { key, entries }
    }
}
impl Slice {
    pub fn new(line: &Arc<Line>, index: usize, count: usize) -> Self {
        Self {
            line: line.clone(),
            index,
            count,
        }
    }
    pub fn new_arc(line: &Arc<Line>, index: usize, count: usize) -> Arc<Self> {
        Arc::new(Self::new(line, index, count))
    }
}
impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>({} E:{})",
            stringify!(Line),
            self.key,
            self.entries.len()
        )
    }
}
impl Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>({} {}-{})",
            stringify!($silce_type),
            self.line.key,
            self.index,
            self.count
        )
    }
}
impl From<notation_proto::prelude::Line> for Line {
    fn from(v: notation_proto::prelude::Line) -> Self {
        let entries: Vec<Arc<ProtoEntry>> =
            v.entries.into_iter().map(|entry| Arc::new(entry)).collect();
        Self::new(v.key, entries)
    }
}
impl TryFrom<(notation_proto::prelude::Slice, &Vec<Arc<Line>>)> for Slice {
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(v: (notation_proto::prelude::Slice, &Vec<Arc<Line>>)) -> Self {
        if let Some(line) = v.1.iter().find(|x| x.key == v.0.line) {
            Self::new(line, v.0.index, v.0.count)
        } else {
            throw!(ParseError::LineNotFound(v.0.line));
        }
    }
}
