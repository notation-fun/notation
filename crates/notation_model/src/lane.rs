use std::{fmt::Display, sync::Weak};
use std::sync::Arc;

use notation_proto::prelude::{FrettedEntry6, ProtoEntry, TrackKind};

use crate::prelude::{LaneEntry, ModelEntry, Slice, TabBar, Track};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LaneKind {
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

#[derive(Debug)]
pub struct BarLane {
    pub bar: Weak<TabBar>,
    pub kind: LaneKind,
    pub track: Arc<Track>,
    pub slice: Slice,
    pub entries: Vec<Arc<LaneEntry>>,
}
impl Display for BarLane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLane>({} {}, E:{})", self.id(), self.slice, self.entries.len())
    }
}

impl BarLane {
    pub fn id(&self) -> String {
        format!("{}:{}", self.track.id, self.kind)
    }
    pub fn try_new_arc(
        bar: Weak<TabBar>,
        track: &Arc<Track>,
        slice: Slice,
    ) -> Option<Arc<Self>> {
        let entries = track.get_entries(&slice.begin, &slice.end);
        if let Some(kind) = LaneKind::of_entries(&track.kind, &entries) {
            Some(Arc::<Self>::new_cyclic(|weak_self| {
                let entries = LaneEntry::new_entries(entries, weak_self);
                Self {
                    bar,
                    kind,
                    track: track.clone(),
                    slice,
                    entries,
                }
            }))
        } else {
            None
        }
    }
}
