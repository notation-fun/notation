use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::Bar;

// https://www.masterclass.com/articles/songwriting-101-learn-common-song-structures
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum SectionKind {
    Ready,
    Intro,
    Verse,
    Chorus,
    Bridge,
    Outro,
    PreChorus,
    Solo,
    Custom(String),
}
impl Display for SectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl SectionKind {
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "Ready" => Self::Ready,
            "Intro" => Self::Intro,
            "Verse" => Self::Verse,
            "Chorus" => Self::Chorus,
            "Bridge" => Self::Bridge,
            "Outro" => Self::Outro,
            "PreChorus" => Self::PreChorus,
            "Solo" => Self::Solo,
            _ => Self::Custom(ident.to_string()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Section {
    pub id: String,
    pub kind: SectionKind,
    pub bars: Vec<Bar>,
}
impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Section>({} <{}> B:{})",
            self.id,
            self.kind,
            self.bars.len()
        )
    }
}
impl Section {
    pub const READY_ID: &'static str = "READY";
    pub fn new(id: String, kind: SectionKind, bars: Vec<Bar>) -> Self {
        Self { id, kind, bars }
    }
    pub fn new_ready() -> Self {
        let mut bars = Vec::new();
        bars.push(Bar { layers: Vec::new() });
        Self::new(Self::READY_ID.to_string(), SectionKind::Ready, bars)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Form {
    pub sections: Vec<String>,
}
impl Display for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Form>(S:{})", self.sections.len())
    }
}
impl From<Vec<String>> for Form {
    fn from(v: Vec<String>) -> Self {
        Self { sections: v }
    }
}
impl From<Vec<&str>> for Form {
    fn from(v: Vec<&str>) -> Self {
        Self {
            sections: v.iter().map(|x| x.to_string()).collect(),
        }
    }
}
