use std::fmt::Display;
use std::sync::Arc;

use notation_proto::prelude::{FrettedEntry6, ProtoEntry, TrackKind};

use crate::prelude::ModelEntry;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LaneKind {
    Meta,
    Chord,
    Lyrics,
    Melody,
    Harmany,
    Keyboard,
    Strings,
    Shapes,
}
impl Display for LaneKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl LaneKind {
    pub fn of_entry(track_kind: &TrackKind, entry: &ProtoEntry) -> Option<Self> {
        match track_kind {
            TrackKind::Meta => Some(Self::Meta),
            TrackKind::Chord => Some(Self::Chord),
            TrackKind::Lyrics => Some(Self::Lyrics),
            TrackKind::Vocal => Some(Self::Melody),
            TrackKind::Guitar => match entry {
                ProtoEntry::Fretted6(entry) => match entry {
                    FrettedEntry6::Pick(_, _) => Some(Self::Strings),
                    FrettedEntry6::Strum(_, _) => Some(Self::Strings),
                    FrettedEntry6::Shape(_, _) => Some(Self::Shapes),
                    FrettedEntry6::Fretboard(_) => None,
                },
                _ => None,
            },
            TrackKind::Synth => Some(Self::Keyboard),
            TrackKind::Piano => Some(Self::Keyboard),
            TrackKind::Drums => None,
            TrackKind::Bass => None,
            TrackKind::Custom(_) => Some(Self::Harmany),
        }
    }
    pub fn of_entries(track_kind: &TrackKind, entries: &Vec<Arc<ModelEntry>>) -> Option<LaneKind> {
        for entry in entries.iter() {
            if let Some(lane) = Self::of_entry(track_kind, &entry.proto) {
                return Some(lane);
            }
        }
        None
    }
}
