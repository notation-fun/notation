use std::rc::Rc;
use std::iter::Iterator;

use crate::prelude::{Unit, Entry, Duration, EntryWrap, ZeroEntryWrap};

#[derive(Clone)]
pub struct Line {
    pub entries: Vec<Rc<dyn Entry>>,
}

pub struct Slice {
    pub line: Rc<Line>,
    pub index: usize,
    pub count: usize,
}

impl Line {
    pub fn new(entries: Vec<Rc<dyn Entry>>) -> Self {
        Self {entries}
    }

    pub fn get_slice(self, index: usize, count: usize) -> Slice {
        Slice {
            line: Rc::new(self),
            index,
            count,
        }
    }
}

impl<T: Copy + Eq> From<Vec<EntryWrap<T>>> for Line {
    fn from(val: Vec<EntryWrap<T>>) -> Self {
        Self::new(val.iter().map(|entry| Box::new(*entry).into()).collect())
    }
}

impl Line {
    pub fn from_iterator<T: Copy + Eq>(iter: impl Iterator<Item=EntryWrap<T>>) -> Self {
        iter.collect::<Vec<EntryWrap<T>>>().into()
    }
}

impl<T: Copy + Eq> From<Vec<(T, Duration)>> for Line {
    fn from(val: Vec<(T, Duration)>) -> Self {
        Self::from_iterator(val.iter().map(|(content, duration)| {
            EntryWrap::<T>::new(*content, *duration)
        }))
    }
}

impl<T: Copy + Eq> From<Vec<(T, Unit)>> for Line {
    fn from(val: Vec<(T, Unit)>) -> Self {
        Self::from_iterator(val.iter().map(|(content, duration)| {
            EntryWrap::<T>::new(*content, (*duration).into())
        }))
    }
}

impl<T: Copy + Eq> From<(Vec<T>, Duration)> for Line {
    fn from((val, duration): (Vec<T>, Duration)) -> Self {
        Self::from_iterator(val.iter().map(|content| {
            EntryWrap::<T>::new(*content, duration)
        }))
    }
}

impl<T: Copy + Eq> From<Vec<ZeroEntryWrap<T>>> for Line {
    fn from(val: Vec<ZeroEntryWrap<T>>) -> Self {
        Self {
            entries: val.iter().map(|entry| Box::new(*entry).into()
            ).collect(),
        }
    }
}

impl Line {
    pub fn from_zero_iterator<T: Copy + Eq>(iter: impl Iterator<Item=ZeroEntryWrap<T>>) -> Self {
        iter.collect::<Vec<ZeroEntryWrap<T>>>().into()
    }
}

impl Line {
    pub fn from_zero<T: Copy + Eq>(val: Vec<T>) -> Self {
        Self::from_zero_iterator(val.iter().map(|content| {
            ZeroEntryWrap::<T>::new(*content)
        }))
    }
}

impl From<Vec<Line>> for Line {
    fn from(val: Vec<Line>) -> Self {
        let mut entries = Vec::<Rc<dyn Entry>>::new();
        for x in val {
            entries.append(&mut x.entries.clone());
        }
        Self::new(entries)
    }
}

impl Line {
    pub fn from_lines_iterator(iter: impl Iterator<Item=Line>) -> Self {
        iter.collect::<Vec<Line>>().into()
    }
}

