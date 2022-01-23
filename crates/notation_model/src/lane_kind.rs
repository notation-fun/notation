use std::fmt::Display;
use std::sync::Arc;

use notation_proto::prelude::{FrettedEntry6, ProtoEntry, TrackKind};

use crate::prelude::ModelEntry;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LaneKind {
    None,
    Meta,
    Chord,
    Lyrics,
    Melody,
    Harmony,
    Keyboard,
    Shapes,
    Strings,
}
impl Display for LaneKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for LaneKind {
    fn default() -> Self {
        Self::None
    }
}
impl LaneKind {
    pub const LEN: usize = 9;
    pub fn order(&self) -> usize {
        match self {
            LaneKind::None => 0,
            LaneKind::Meta => 1,
            LaneKind::Chord => 2,
            LaneKind::Lyrics => 3,
            LaneKind::Melody => 4,
            LaneKind::Harmony => 5,
            LaneKind::Keyboard => 6,
            LaneKind::Shapes => 7,
            LaneKind::Strings => 8,
        }
    }
    pub fn of_entry(track_kind: &TrackKind, entry: &ProtoEntry) -> Self {
        match track_kind {
            TrackKind::Unsupported => Self::None,
            TrackKind::Meta => Self::Meta,
            TrackKind::Chord => Self::Chord,
            TrackKind::Lyrics => Self::Lyrics,
            TrackKind::Vocal => Self::Melody,
            TrackKind::Guitar => match entry {
                ProtoEntry::Fretted6(entry) => match entry {
                    FrettedEntry6::Pick(_, _) => Self::Strings,
                    FrettedEntry6::Strum(_, _) => Self::Strings,
                    FrettedEntry6::Shape(_, _) => Self::Shapes,
                    FrettedEntry6::Fretboard(_) => Self::None,
                },
                _ => Self::None,
            },
            TrackKind::Synth => Self::Keyboard,
            TrackKind::Piano => Self::Keyboard,
            TrackKind::Drums => Self::None,
            TrackKind::Bass => Self::None,
        }
    }
    pub fn of_entries(track_kind: &TrackKind, entries: &Vec<Arc<ModelEntry>>) -> LaneKind {
        for entry in entries.iter() {
            let kind = Self::of_entry(track_kind, &entry.proto);
            if kind != Self::None {
                return kind;
            }
        }
        Self::None
    }

    /// Returns `true` if the lane kind is [`None`].
    ///
    /// [`None`]: LaneKind::None
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the lane kind is [`Meta`].
    ///
    /// [`Meta`]: LaneKind::Meta
    pub fn is_meta(&self) -> bool {
        matches!(self, Self::Meta)
    }

    /// Returns `true` if the lane kind is [`Chord`].
    ///
    /// [`Chord`]: LaneKind::Chord
    pub fn is_chord(&self) -> bool {
        matches!(self, Self::Chord)
    }

    /// Returns `true` if the lane kind is [`Lyrics`].
    ///
    /// [`Lyrics`]: LaneKind::Lyrics
    pub fn is_lyrics(&self) -> bool {
        matches!(self, Self::Lyrics)
    }

    /// Returns `true` if the lane kind is [`Melody`].
    ///
    /// [`Melody`]: LaneKind::Melody
    pub fn is_melody(&self) -> bool {
        matches!(self, Self::Melody)
    }

    /// Returns `true` if the lane kind is [`Harmony`].
    ///
    /// [`Harmony`]: LaneKind::Harmony
    pub fn is_harmony(&self) -> bool {
        matches!(self, Self::Harmony)
    }

    /// Returns `true` if the lane kind is [`Keyboard`].
    ///
    /// [`Keyboard`]: LaneKind::Keyboard
    pub fn is_keyboard(&self) -> bool {
        matches!(self, Self::Keyboard)
    }

    /// Returns `true` if the lane kind is [`Strings`].
    ///
    /// [`Strings`]: LaneKind::Strings
    pub fn is_strings(&self) -> bool {
        matches!(self, Self::Strings)
    }

    /// Returns `true` if the lane kind is [`Shapes`].
    ///
    /// [`Shapes`]: LaneKind::Shapes
    pub fn is_shapes(&self) -> bool {
        matches!(self, Self::Shapes)
    }
}
