use std::fmt::Display;
use std::iter::Iterator;
use std::sync::Arc;

use crate::prelude::ProtoEntry;

#[derive(Debug)]
pub struct Line {
    pub name: String,
    pub entries: Vec<Arc<ProtoEntry>>,
}
#[derive(Debug)]
pub struct Slice {
    pub line: Arc<Line>,
    pub index: usize,
    pub count: usize,
}
impl Line {
    pub fn new(name: String, entries: Vec<Arc<ProtoEntry>>) -> Self {
        Self { name, entries }
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
            self.name,
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
            self.line.name,
            self.index,
            self.count
        )
    }
}
impl From<(String, Vec<ProtoEntry>)> for Line {
    fn from(v: (String, Vec<ProtoEntry>)) -> Self {
        let entries: Vec<Arc<ProtoEntry>> = v.1.into_iter().map(|entry| Arc::new(entry)).collect();
        Self::new(v.0, entries)
    }
}
impl From<(String, Vec<Arc<ProtoEntry>>)> for Line {
    fn from(v: (String, Vec<Arc<ProtoEntry>>)) -> Self {
        Self::new(v.0, v.1)
    }
}
impl Line {
    pub fn from_iterator(name: String, iter: impl Iterator<Item = ProtoEntry>) -> Self {
        Self::from((name, iter.collect::<Vec<ProtoEntry>>()))
    }
    pub fn from_entries(name: String, iter: impl Iterator<Item = Arc<ProtoEntry>>) -> Self {
        Self::from((name, iter.collect::<Vec<Arc<ProtoEntry>>>()))
    }
}
impl From<(String, Vec<Line>)> for Line {
    fn from(v: (String, Vec<Line>)) -> Self {
        let mut entries = Vec::<Arc<ProtoEntry>>::new();
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
