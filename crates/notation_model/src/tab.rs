use std::fmt::Display;
use std::sync::Arc;

use notation_proto::prelude::{Note, SyllableNote};

use crate::prelude::{
    Form, Pitch, Section, Signature, Syllable, TabBar, TabMeta, Track, Unit, Units,
};

#[derive(Debug)]
pub struct Tab {
    pub meta: Arc<TabMeta>,
    pub tracks: Vec<Arc<Track>>,
    pub sections: Vec<Arc<Section>>,
    pub form: Form,
    pub bars: Vec<Arc<TabBar>>,
}
impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>({} T:{} S:{} F:{} B:{})",
            stringify!(Tab),
            self.meta,
            self.tracks.len(),
            self.sections.len(),
            self.form.sections.len(),
            self.bars.len(),
        )
    }
}
impl Tab {
    pub fn bar_units(&self) -> Units {
        self.meta.bar_units()
    }
    pub fn bar_beats(&self) -> u8 {
        self.meta.signature.bar_beats
    }
    pub fn signature(&self) -> Signature {
        self.meta.signature
    }
    pub fn beat_unit(&self) -> Unit {
        self.meta.signature.beat_unit
    }
    pub fn calc_syllable(&self, pitch: &Pitch) -> Syllable {
        self.meta.calc_syllable(pitch)
    }
    pub fn calc_syllable_note(&self, note: &Note) -> SyllableNote {
        self.meta.calc_syllable_note(note)
    }
}
