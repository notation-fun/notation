use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::prelude::{BarLayer, Form, Line, Section, Track};
use notation_core::prelude::{Key, Pitch, Scale, Signature, Syllable, Tempo};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TabMeta {
    pub key: Key,
    pub scale: Scale,
    pub signature: Signature,
    pub tempo: Tempo,
}
impl TabMeta {
    pub fn calc_syllable(&self, pitch: &Pitch) -> Syllable {
        self.scale.calc_syllable(&self.key, pitch)
    }
}
impl Display for TabMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {:?}",
            self.key, self.scale, self.signature, self.tempo,
        )
    }
}
impl TabMeta {
    pub fn new(key: Key, scale: Scale, signature: Signature, tempo: Tempo) -> Self {
        Self {
            key,
            scale,
            signature,
            tempo,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tab {
    pub meta: TabMeta,
    pub lines: Vec<Line>,
    pub tracks: Vec<Track>,
    pub layers: Vec<BarLayer>,
    pub sections: Vec<Section>,
    pub form: Form,
}
impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Tab>({} L:{} T:{} Y:{} S:{} F:{})",
            self.meta,
            self.lines.len(),
            self.tracks.len(),
            self.layers.len(),
            self.sections.len(),
            self.form.sections.len(),
        )
    }
}
impl Tab {
    pub fn new(
        meta: TabMeta,
        lines: Vec<Line>,
        tracks: Vec<Track>,
        layers: Vec<BarLayer>,
        sections: Vec<Section>,
        form: Form,
    ) -> Self {
        Self {
            meta,
            lines,
            tracks,
            layers,
            sections,
            form,
        }
    }
}
