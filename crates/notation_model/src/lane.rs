use std::fmt::Display;
use std::sync::Arc;

use notation_proto::prelude::{FrettedEntry, ProtoEntry, TrackKind};

use crate::prelude::Slice;

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
    pub fn calc_lane_kind(track: &TrackKind, entry: &ProtoEntry) -> Option<Self> {
        match track {
            TrackKind::Chord => Some(Self::Chord),
            TrackKind::Lyrics => Some(Self::Lyrics),
            TrackKind::Vocal => Some(Self::Melody),
            TrackKind::Guitar => match entry {
                ProtoEntry::FrettedSix(entry) => match entry {
                    FrettedEntry::Pick(_, _) => Some(Self::Strings),
                    FrettedEntry::Strum(_, _) => Some(Self::Strings),
                    FrettedEntry::Shape(_, _) => Some(Self::Shapes),
                    FrettedEntry::Fretboard(_) => None,
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
}

#[derive(Debug)]
pub struct BarLane {
    pub kind: LaneKind,
    pub slice: Arc<Slice>,
}
impl Display for BarLane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLane>({} {})", self.kind, self.slice)
    }
}

impl BarLane {
    pub fn id(&self) -> String {
        format!("{}:{}", self.slice.track.id, self.kind)
    }
    pub fn try_from_slice(slice: Arc<Slice>) -> Option<Self> {
        slice.calc_lane_kind().map(|kind| BarLane { kind, slice })
    }
    pub fn not_in_round(&self, round: usize) -> bool {
        self.slice.rounds.is_some()
            && self
                .slice
                .rounds
                .clone()
                .unwrap()
                .iter()
                .find(|&x| *x == round)
                .is_none()
    }
    pub fn in_round(&self, round: usize) -> bool {
        !self.not_in_round(round)
    }
}
