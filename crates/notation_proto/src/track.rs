use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::prelude::ProtoEntry;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TrackKind {
    Vocal,
    Lyrics,
    Guitar,
    Synth,
    Piano,
    Drums,
    Bass,
    Custom(String),
}
impl Display for TrackKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl TrackKind {
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "Vocal" => Self::Vocal,
            "Lyrics" => Self::Lyrics,
            "Guitar" => Self::Guitar,
            "Synth" => Self::Synth,
            "Piano" => Self::Piano,
            "Drums" => Self::Drums,
            "Bass" => Self::Bass,
            _ => Self::Custom(ident.to_string()),
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
