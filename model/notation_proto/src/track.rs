use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::prelude::ProtoEntry;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TrackKind {
    Unsupported,
    Meta,
    Chord,
    Lyrics,
    Vocal,
    Guitar,
    Synth,
    Piano,
    Drums,
    Bass,
}
impl Display for TrackKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl TrackKind {
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "Meta" => Self::Meta,
            "Chord" => Self::Chord,
            "Lyrics" => Self::Lyrics,
            "Vocal" => Self::Vocal,
            "Guitar" => Self::Guitar,
            "Synth" => Self::Synth,
            "Piano" => Self::Piano,
            "Drums" => Self::Drums,
            "Bass" => Self::Bass,
            _ => {
                println!("TrackKind::from_ident() Unsupported ident: {}", ident);
                Self::Unsupported
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Track {
    pub id: String,
    pub kind: TrackKind,
    pub entries: Vec<ProtoEntry>,
}
impl Track {
    pub fn new(id: String, kind: TrackKind, entries: Vec<ProtoEntry>) -> Self {
        Self { kind, id, entries }
    }
}
impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Track>({} <{}> E:{})",
            self.id,
            self.kind,
            self.entries.len()
        )
    }
}
