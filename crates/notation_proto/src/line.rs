use std::rc::Rc;
use std::sync::Arc;
use std::iter::Iterator;

use notation_core::prelude::{Unit, Duration};
use crate::prelude::{Entry, EntryWrap, ZeroEntryWrap};

macro_rules! impl_line_slice {
    ($ref_type:ident, $line_name:ident, $slice_name:ident) => {
        #[derive(Clone)]
        pub struct $line_name {
            pub entries: Vec<$ref_type<dyn Entry>>,
        }

        pub struct $slice_name {
            pub line: $ref_type<$line_name>,
            pub index: usize,
            pub count: usize,
        }

        impl $line_name {
            pub fn new(entries: Vec<$ref_type<dyn Entry>>) -> Self {
                Self {entries}
            }

            pub fn get_slice(self, index: usize, count: usize) -> $slice_name {
                $slice_name {
                    line: $ref_type::new(self),
                    index,
                    count,
                }
            }
        }

        impl<T: Copy + Eq> From<Vec<EntryWrap<T>>> for $line_name {
            fn from(val: Vec<EntryWrap<T>>) -> Self {
                Self::new(val.iter().map(|entry| Box::new(*entry).into()).collect())
            }
        }

        impl From<Vec<$ref_type<dyn Entry>>> for $line_name {
            fn from(val: Vec<$ref_type<dyn Entry>>) -> Self {
                Self::new(val)
            }
        }

        impl $line_name {
            pub fn from_iterator<T: Copy + Eq>(iter: impl Iterator<Item=EntryWrap<T>>) -> Self {
                iter.collect::<Vec<EntryWrap<T>>>().into()
            }
            pub fn from_entries(iter: impl Iterator<Item=$ref_type<dyn Entry>>) -> Self {
                iter.collect::<Vec<$ref_type<dyn Entry>>>().into()
            }
        }

        impl<T: Copy + Eq> From<Vec<(T, Duration)>> for $line_name {
            fn from(val: Vec<(T, Duration)>) -> Self {
                Self::from_iterator(val.iter().map(|(content, duration)| {
                    EntryWrap::<T>::new(*content, *duration)
                }))
            }
        }

        impl<T: Copy + Eq> From<Vec<(T, Unit)>> for $line_name {
            fn from(val: Vec<(T, Unit)>) -> Self {
                Self::from_iterator(val.iter().map(|(content, duration)| {
                    EntryWrap::<T>::new(*content, (*duration).into())
                }))
            }
        }

        impl<T: Copy + Eq> From<(Vec<T>, Duration)> for $line_name {
            fn from((val, duration): (Vec<T>, Duration)) -> Self {
                Self::from_iterator(val.iter().map(|content| {
                    EntryWrap::<T>::new(*content, duration)
                }))
            }
        }

        impl<T: Copy + Eq> From<Vec<ZeroEntryWrap<T>>> for $line_name {
            fn from(val: Vec<ZeroEntryWrap<T>>) -> Self {
                Self {
                    entries: val.iter().map(|entry| Box::new(*entry).into()
                    ).collect(),
                }
            }
        }

        impl $line_name {
            pub fn from_zero_iterator<T: Copy + Eq>(iter: impl Iterator<Item=ZeroEntryWrap<T>>) -> Self {
                iter.collect::<Vec<ZeroEntryWrap<T>>>().into()
            }
        }

        impl $line_name {
            pub fn from_zero<T: Copy + Eq>(val: Vec<T>) -> Self {
                Self::from_zero_iterator(val.iter().map(|content| {
                    ZeroEntryWrap::<T>::new(*content)
                }))
            }
        }

        impl From<Vec<$line_name>> for $line_name {
            fn from(val: Vec<$line_name>) -> Self {
                let mut entries = Vec::<$ref_type<dyn Entry>>::new();
                for x in val {
                    entries.append(&mut x.entries.clone());
                }
                Self::new(entries)
            }
        }

        impl $line_name {
            pub fn from_lines_iterator(iter: impl Iterator<Item=$line_name>) -> Self {
                iter.collect::<Vec<$line_name>>().into()
            }
        }
    }
}

impl_line_slice!(Rc, RcLine, RcSlice);
impl_line_slice!(Arc, ArcLine, ArcSlice);
