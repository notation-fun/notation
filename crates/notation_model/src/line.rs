use fehler::{throw, throws};
use notation_proto::prelude::{SliceBegin, SliceEnd};
use std::convert::TryFrom;
use std::fmt::Display;
use std::iter::Iterator;
use std::sync::Arc;

use crate::prelude::{ParseError, ProtoEntry};

#[derive(Debug)]
pub struct Line {
    pub id: String,
    pub entries: Vec<Arc<ProtoEntry>>,
}
#[derive(Debug)]
pub struct Slice {
    pub line: Arc<Line>,
    pub begin: SliceBegin,
    pub end: SliceEnd,
    pub entries: Vec<Arc<ProtoEntry>>,
}
impl Line {
    pub fn new(id: String, entries: Vec<Arc<ProtoEntry>>) -> Self {
        Self { id, entries }
    }
    pub fn index_of_mark(&self, begin: usize, mark: &String) -> Option<usize> {
        for i in begin..self.entries.len() {
            let entry = self.entries.get(i);
            if entry.is_some() && entry.unwrap().is_mark_string(mark) {
                return Some(i);
            }
        }
        None
    }
    pub fn get_entries(&self, begin: &SliceBegin, end: &SliceEnd) -> Vec<Arc<ProtoEntry>> {
        let (index, count) = match (begin, end) {
            (SliceBegin::Mark(x), SliceEnd::Mark(y)) => match self.index_of_mark(0, x) {
                Some(index) => {
                    let index = index + 1;
                    match self.index_of_mark(index, y) {
                        Some(end) => (index, end - index),
                        None => (index, 0),
                    }
                }
                None => (0, 0),
            },
            (SliceBegin::Mark(x), SliceEnd::Count(y)) => match self.index_of_mark(0, x) {
                Some(index) => (index + 1, *y),
                None => (0, 0),
            },
            (SliceBegin::Index(x), SliceEnd::Mark(y)) => match self.index_of_mark(*x, y) {
                Some(end) => (*x, end - 1 - *x),
                None => (*x, 0),
            },
            (SliceBegin::Index(x), SliceEnd::Count(y)) => (*x, *y),
        };
        let mut entries = vec![];
        for i in index..(index + count) {
            let entry = self.entries.get(i);
            if entry.is_some() {
                entries.push(entry.unwrap().clone());
            }
        }
        entries
    }
}
impl Slice {
    pub fn new(line: &Arc<Line>, begin: SliceBegin, end: SliceEnd) -> Self {
        let entries = line.get_entries(&begin, &end);
        Self {
            line: line.clone(),
            begin,
            end,
            entries,
        }
    }
    pub fn new_arc(line: &Arc<Line>, begin: SliceBegin, end: SliceEnd) -> Arc<Self> {
        Arc::new(Self::new(line, begin, end))
    }
}
impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>({} E:{})",
            stringify!(Line),
            self.id,
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
            self.line.id,
            self.begin,
            self.end,
        )
    }
}
impl From<notation_proto::prelude::Line> for Line {
    fn from(v: notation_proto::prelude::Line) -> Self {
        let entries: Vec<Arc<ProtoEntry>> = v.entries.into_iter().map(Arc::new).collect();
        Self::new(v.id, entries)
    }
}
impl TryFrom<(notation_proto::prelude::Slice, &Vec<Arc<Line>>)> for Slice {
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(v: (notation_proto::prelude::Slice, &Vec<Arc<Line>>)) -> Self {
        if let Some(line) = v.1.iter().find(|x| x.id == v.0.line) {
            Self::new(line, v.0.begin, v.0.end)
        } else {
            throw!(ParseError::LineNotFound(v.0.line));
        }
    }
}
