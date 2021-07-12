use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::iter::Iterator;

use crate::prelude::ProtoEntry;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Line {
    pub key: String,
    pub entries: Vec<ProtoEntry>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Slice {
    pub line: String,
    pub index: usize,
    pub count: usize,
}
impl Line {
    pub fn new(key: String, entries: Vec<ProtoEntry>) -> Self {
        Self { key, entries }
    }
}
impl Slice {
    pub fn new(line: String, index: usize, count: usize) -> Self {
        Self {
            line: line,
            index,
            count,
        }
    }
}
impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Line>({} E:{})", self.key, self.entries.len())
    }
}
impl Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Slice>({} {}-{})", self.line, self.index, self.count)
    }
}
impl From<(String, Vec<ProtoEntry>)> for Line {
    fn from(v: (String, Vec<ProtoEntry>)) -> Self {
        Self::new(v.0, v.1)
    }
}
impl Line {
    pub fn from_iterator(name: String, iter: impl Iterator<Item = ProtoEntry>) -> Self {
        Self::from((name, iter.collect::<Vec<ProtoEntry>>()))
    }
}
impl From<(String, Vec<Line>)> for Line {
    fn from(v: (String, Vec<Line>)) -> Self {
        let mut entries = Vec::<ProtoEntry>::new();
        for x in v.1 {
            entries.append(&mut x.entries.clone());
        }
        Self::new(v.0, entries)
    }
}
impl Line {
    pub fn from_lines_iterator(name: String, iter: impl Iterator<Item = Line>) -> Self {
        Self::from((name, iter.collect::<Vec<Line>>()))
    }
}
