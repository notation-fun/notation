use std::{collections::HashMap, fmt::Display};
use std::sync::Arc;

use notation_proto::prelude::Chord;

use crate::prelude::{Fretboard4, Fretboard6, ModelEntry, SliceBegin, SliceEnd, TrackKind};

#[derive(Copy, Clone, Debug, Default)]
pub struct TrackProps {
    pub index: usize,
}

#[derive(Debug)]
pub struct Track {
    pub id: String,
    pub kind: TrackKind,
    pub entries: Vec<Arc<ModelEntry>>,
    pub props: TrackProps,
}
impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Track>({:?}, {}, [{}])",
            self.kind,
            self.id,
            self.entries.len()
        )
    }
}
impl Track {
    pub fn new(index: usize, id: String, kind: TrackKind, entries: Vec<Arc<ModelEntry>>) -> Self {
        let props = TrackProps {
            index,
        };
        Self { id, kind, entries, props }
    }
    pub fn new_arc(index: usize, v: notation_proto::prelude::Track) -> Arc<Self> {
        Arc::<Self>::new_cyclic(|weak_self| {
            let entries = ModelEntry::new_entries(v.entries, weak_self);
            Self::new(index, v.id, v.kind, entries)
        })
    }
}
impl Track {
    pub fn index_of_mark(&self, begin: usize, mark: &String) -> Option<usize> {
        for i in begin..self.entries.len() {
            let entry = self.entries.get(i);
            if entry.is_some() && entry.unwrap().proto.is_mark_string(mark) {
                return Some(i);
            }
        }
        None
    }
    pub fn get_entries(&self, begin: &SliceBegin, end: &SliceEnd) -> Vec<Arc<ModelEntry>> {
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
    pub fn get_chords(&self) -> Vec<(Arc<ModelEntry>, Chord)> {
        let mut chords = Vec::new();
        for entry in self.entries.iter() {
            if let Some(chord) = entry.proto
                .as_core()
                .and_then(|x| x.as_chord())
                .map(|z|z.to_owned()) {
                chords.push((entry.clone(), chord));
            }
        }
        chords
    }
    pub fn get_unique_chords(&self) -> Vec<(Chord, Arc<ModelEntry>)> {
        let chords = self.get_chords();
        if chords.len() == 0 {
            return Vec::new();
        }
        let mut unique_chords: HashMap<Chord, Arc<ModelEntry>> = HashMap::new();
        for (entry, chord) in chords.into_iter() {
            if !unique_chords.contains_key(&chord) {
                unique_chords.insert(chord, entry);
            }
        }
        let mut chords = unique_chords.into_iter()
            .collect::<Vec<(Chord, Arc<ModelEntry>)>>();
        chords.sort_by(|(a, _), (b, _)| {
                a.cmp(&b)
            });
        chords
    }
}

macro_rules! impl_get_fretboard {
    ($name:ident, $strings:literal, $as_fretted:ident, $fretboard:ident) => {
        impl Track {
            pub fn $name(&self) -> Option<$fretboard> {
                self.get_entry(&|x: &ModelEntry| {
                    x.$as_fretted()
                        .and_then(|x| x.as_fretboard().map(|z| z.to_owned()))
                })
            }
        }
    };
}

impl_get_fretboard!(get_fretboard6, 6, as_fretted6, Fretboard6);
impl_get_fretboard!(get_fretboard4, 4, as_fretted4, Fretboard4);
