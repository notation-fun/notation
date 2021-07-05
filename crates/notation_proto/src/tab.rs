use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Weak};

use crate::prelude::{Bar, Line, Section, Track};
use notation_core::prelude::{
    Key, Note, Scale, Semitones, Signature, Syllable, Tempo, Unit, Units,
};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TabMeta {
    pub key: Key,
    pub scale: Scale,
    pub signature: Signature,
    pub tempo: Tempo,
}
impl TabMeta {
    pub fn calc_syllable(&self, note: &Note) -> Syllable {
        self.scale.calc_syllable(&self.key, note)
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
    pub sections: Vec<Arc<Section>>,
    pub form: Vec<usize>,
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
            "<{}>({} L:{} T:{} S:{} F:{} B:{})",
            stringify!(Tab),
            self.meta,
            self.lines.len(),
            self.tracks.len(),
            self.sections.len(),
            self.form.len(),
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
impl Section {
    pub fn new_tab_bars(
        &self,
        arc_section: Arc<Section>,
        tab: Weak<Tab>,
        section_round: usize,
        section_ordinal: usize,
        section_bar_ordinal: usize,
    ) -> Vec<Arc<TabBar>> {
        self.bars
            .iter()
            .enumerate()
            .map(|(bar_index, bar)| TabBar {
                tab: tab.clone(),
                section: arc_section.clone(),
                section_round,
                section_ordinal,
                bar: bar.clone(),
                bar_index: bar_index,
                bar_ordinal: section_bar_ordinal + bar_index,
            })
            .map(|x| Arc::new(x))
            .collect()
    }
}
impl Tab {
    pub fn new(
        meta: Arc<TabMeta>,
        lines: Vec<Arc<Line>>,
        tracks: Vec<Arc<Track>>,
        sections: Vec<Arc<Section>>,
        form: Vec<usize>,
    ) -> Arc<Self> {
        Arc::<Tab>::new_cyclic(|weak_self| {
            let bars = Self::new_tab_bars(weak_self, &sections, &form);
            Self {
                meta,
                lines,
                tracks,
                sections,
                form,
                bars,
            }
        })
    }
    fn new_tab_bars(
        weak_self: &Weak<Tab>,
        sections: &Vec<Arc<Section>>,
        form: &Vec<usize>,
    ) -> Vec<Arc<TabBar>> {
        let mut section_rounds: HashMap<usize, usize> = HashMap::new();
        let mut section_ordinal: usize = 1;
        let mut bar_ordinal: usize = 1;
        let mut bars: Vec<Arc<TabBar>> = vec![];
        for (form_index, section_index) in form.iter().enumerate() {
            if *section_index <= sections.len() {
                let section = sections.get(*section_index).unwrap();
                let section_round = match section_rounds.get(section_index) {
                    Some(r) => r + 1,
                    None => 1,
                };
                section_rounds.insert(*section_index, section_round);
                bars.extend(section.new_tab_bars(
                    section.clone(),
                    weak_self.clone(),
                    section_round,
                    section_ordinal,
                    bar_ordinal,
                ));
                section_ordinal += 1;
                bar_ordinal += section.bars.len();
            } else {
                println!(
                    "Invalid section_index in form: {:?} -> {:?}",
                    form_index, *section_index
                );
            }
        }
        println!("new_tab_bars() -> {:?} bars", bars.len());
        bars
    }
}
