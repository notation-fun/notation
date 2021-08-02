use std::fmt::Display;
use std::sync::{Arc, Weak};

use notation_proto::prelude::TabPosition;

use crate::prelude::{
    Bar, Form, Pitch, Section, Semitones, Signature, Syllable, TabMeta, Track,
    Unit, Units,
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
        Units::from(self.meta.signature)
    }
    pub fn bar_beats(&self) -> u8 {
        self.meta.signature.beats_per_bar
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
}

impl TabBar {
    pub fn tab_pos(&self) -> TabPosition {
        TabPosition::new(Units((self.bar_ordinal - 1) as f32 * self.bar_units().0))
    }
    pub fn bar_units(&self) -> Units {
        match self.tab.upgrade() {
            Some(tab) => tab.bar_units(),
            None => {
                println!("<{}>.bar_units() tab missing: {}", stringify!(TabBar), self);
                Units::from(Unit::Whole)
            }
        }
    }
    pub fn bar_beats(&self) -> u8 {
        match self.tab.upgrade() {
            Some(tab) => tab.bar_beats(),
            None => {
                println!("<{}>.bar_beats() tab missing: {}", stringify!(TabBar), self);
                4 as u8
            }
        }
    }
    pub fn signature(&self) -> Signature {
        match self.tab.upgrade() {
            Some(tab) => tab.signature(),
            None => {
                println!("<{}>.signature() tab missing: {}", stringify!(TabBar), self);
                Signature::_4_4
            }
        }
    }
    pub fn beat_unit(&self) -> Unit {
        match self.tab.upgrade() {
            Some(tab) => tab.beat_unit(),
            None => {
                println!("<{}>.beat_unit() tab missing: {}", stringify!(TabBar), self);
                Unit::Quarter
            }
        }
    }
    pub fn calc_syllable(&self, pitch: &Pitch) -> Syllable {
        match self.tab.upgrade() {
            Some(tab) => tab.calc_syllable(pitch),
            None => {
                println!(
                    "<{}>.calc_syllable({}) tab missing: {}",
                    stringify!(TabBar),
                    pitch,
                    self
                );
                Syllable::from(Semitones::from(*pitch))
            }
        }
    }
}
