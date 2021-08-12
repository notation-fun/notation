use std::fmt::Display;
use std::sync::{Arc, Weak};

use notation_proto::prelude::{
    Fretboard4, Fretboard6, HandShape4, HandShape6, Note, SyllableNote, TabPosition,
};

use crate::prelude::{
    Bar, Pitch, Section, Signature, SliceEntry, Syllable, Tab, TabMeta, Unit, Units,
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
macro_rules! impl_get_fretted_shape {
    ($name:ident, $strings:literal, $get_fretted_shape:ident, $fretboard:ident, $hand_shape:ident) => {
        impl TabBar {
            pub fn $name(&self, entry: &SliceEntry) -> Option<($fretboard, $hand_shape)> {
                self.bar.$get_fretted_shape(entry)
            }
        }
    };
}

impl_get_fretted_shape!(
    get_fretted_shape6,
    6,
    get_fretted_shape6,
    Fretboard6,
    HandShape6
);
impl_get_fretted_shape!(
    get_fretted_shape4,
    4,
    get_fretted_shape4,
    Fretboard4,
    HandShape4
);
