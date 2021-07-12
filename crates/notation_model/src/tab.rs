use std::fmt::Display;
use std::sync::{Arc, Weak};

use crate::prelude::{
    Bar, BarLayer, Form, Line, Note, Section, Semitones, Signature, Syllable, TabMeta, Track, Unit,
    Units,
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
    pub lines: Vec<Arc<Line>>,
    pub tracks: Vec<Arc<Track>>,
    pub layers: Vec<Arc<BarLayer>>,
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
            "<{}>({} L:{} T:{} Y:{} S:{} F:{} B:{})",
            stringify!(Tab),
            self.meta,
            self.lines.len(),
            self.tracks.len(),
            self.layers.len(),
            self.sections.len(),
            self.form.sections.len(),
            self.bars.len(),
        )
    }
}
impl TabBar {
    pub fn bar_units(&self) -> Units {
        match self.tab.upgrade() {
            Some(tab) => Units::from(tab.meta.signature),
            None => {
                println!("<{}>.bar_units() tab missing: {}", stringify!(TabBar), self);
                Units::from(Unit::Whole)
            }
        }
    }
    pub fn signature(&self) -> Signature {
        match self.tab.upgrade() {
            Some(tab) => tab.meta.signature.clone(),
            None => {
                println!("<{}>.signature() tab missing: {}", stringify!(TabBar), self);
                Signature::_4_4
            }
        }
    }
    pub fn beat_unit(&self) -> Unit {
        match self.tab.upgrade() {
            Some(tab) => tab.meta.signature.beat_unit,
            None => {
                println!("<{}>.beat_unit() tab missing: {}", stringify!(TabBar), self);
                Unit::Quarter
            }
        }
    }
    pub fn calc_syllable(&self, note: &Note) -> Syllable {
        match self.tab.upgrade() {
            Some(tab) => tab.meta.calc_syllable(note),
            None => {
                println!(
                    "<{}>.calc_syllable({}) tab missing: {}",
                    stringify!(TabBar),
                    note,
                    self
                );
                Syllable::from(Semitones::from(*note))
            }
        }
    }
}
