use std::fmt::Display;
use std::sync::{Arc, Weak};

use notation_proto::prelude::{Note, SyllableNote, TabPosition};

use crate::prelude::{
    Bar, Form, Pitch, Section, Signature, Syllable, TabMeta, Track, Unit, Units,
};

#[derive(Debug)]
pub struct TabBar {
    pub tab: Weak<Tab>,
    pub section: Arc<Section>,
    pub section_round: usize,
    pub section_ordinal: usize,
    pub bar: Arc<Bar>,
    pub bar_index: usize,
    pub bar_ordinal: usize,
}
#[derive(Debug)]
pub struct Tab {
    pub meta: Arc<TabMeta>,
    pub tracks: Vec<Arc<Track>>,
    pub sections: Vec<Arc<Section>>,
    pub form: Form,
    pub bars: Vec<Arc<TabBar>>,
}
impl Display for TabBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>({} {}:{})",
            stringify!(TabBar),
            self.bar_ordinal,
            self.section_ordinal,
            self.bar_index
        )
    }
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

impl TabBar {
    pub fn tab_pos(&self) -> TabPosition {
        TabPosition::new(Units((self.bar_ordinal - 1) as f32 * self.bar_units().0))
    }
    pub fn tab_meta(&self) -> Arc<TabMeta> {
        match self.tab.upgrade() {
            Some(tab) => tab.meta.clone(),
            None => {
                println!("<TabBar>.bar_units() tab_meta missing: {}", self);
                Arc::new(TabMeta::default())
            }
        }
    }
    pub fn bar_units(&self) -> Units {
        self.tab_meta().bar_units()
    }
    pub fn bar_beats(&self) -> u8 {
        self.tab_meta().signature.bar_beats
    }
    pub fn signature(&self) -> Signature {
        self.tab_meta().signature
    }
    pub fn beat_unit(&self) -> Unit {
        self.tab_meta().signature.beat_unit
    }
    pub fn calc_syllable(&self, pitch: &Pitch) -> Syllable {
        self.tab_meta().calc_syllable(pitch)
    }
    pub fn calc_syllable_note(&self, note: &Note) -> SyllableNote {
        self.tab_meta().calc_syllable_note(note)
    }
}
