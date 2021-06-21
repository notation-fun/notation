use std::rc::Rc;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use notation_core::duration::Duration;
use crate::entry::Entry;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EntryWrap<T: Copy + Eq> {
    pub content: T,
    pub duration: Duration,
}

impl<T: Copy + Eq> EntryWrap<T> {
    pub fn new(content: T, duration: Duration) -> Self {
        Self {
            content,
            duration,
        }
    }
}

impl<T: Copy + Eq> Entry for EntryWrap<T> {
    fn duration(&self) -> Duration {
        self.duration
    }
}

impl<T: Copy + Eq> From<(T, Duration)> for EntryWrap<T> {
    fn from((content, duration): (T, Duration)) -> Self {
        Self::new(content, duration)
    }
}

impl<T: Copy + Eq> From<(T, Duration)> for Box<EntryWrap<T>> {
    fn from((content, duration): (T, Duration)) -> Self {
        let wrap = EntryWrap::<T>::new(content, duration);
        Box::new(wrap)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ZeroEntryWrap<T: Copy + Eq> {
    pub content: T,
}

impl<T: Copy + Eq> ZeroEntryWrap<T> {
    pub fn new(content: T) -> Self {
        Self {
            content,
        }
    }
}

impl<T: Copy + Eq> Entry for ZeroEntryWrap<T> {
}

impl<T: Copy + Eq> From<T> for ZeroEntryWrap<T> {
    fn from(content: T) -> Self {
        Self::new(content)
    }
}

impl<T: Copy + Eq> From<T> for Box<ZeroEntryWrap<T>> {
    fn from(content: T) -> Self {
        let wrap = ZeroEntryWrap::<T>::new(content);
        Box::new(wrap)
    }
}

macro_rules! impl_from_ref_for_entry {
    ($ref_type: ident) => {
        impl<T: Copy + Eq> From<Box<EntryWrap<T>>> for $ref_type<dyn Entry> {
            fn from(val: Box<EntryWrap<T>>) -> Self {
                val.into()
            }
        }
        impl<T: Copy + Eq> From<Box<ZeroEntryWrap<T>>> for $ref_type<dyn Entry> {
            fn from(val: Box<ZeroEntryWrap<T>>) -> Self {
                val.into()
            }
        }
    };
}

impl_from_ref_for_entry!(Rc);
impl_from_ref_for_entry!(Arc);
