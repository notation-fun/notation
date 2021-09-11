use fehler::throws;
use std::collections::HashMap;

use std::sync::{Arc, Weak};
use thiserror::Error;

use crate::prelude::{BarLane, Form, LaneEntry, ModelEntry, Section, Tab, TabBar, TabMeta, Track};
use notation_proto::prelude::{Duration, Entry, ProtoEntry, Units};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("track not found")]
    TrackNotFound(String),
    #[error("section not found")]
    SectionNotFound(String),
}

impl Tab {
    #[throws(ParseError)]
    pub fn try_parse_arc(proto: notation_proto::prelude::Tab) -> Arc<Self> {
        Arc::<Tab>::new_cyclic(|weak_self| {
            let uuid = proto.uuid;
            let meta = Arc::new(proto.meta);
            let tracks = proto
                .tracks
                .into_iter()
                .enumerate()
                .map(|(index, track)| Track::new_arc(weak_self.clone(), index, track))
                .collect();
            let mut sections = Vec::new();
            for (index, section) in proto.sections.into_iter().enumerate() {
                let section_id = section.id.clone();
                match Section::try_new(weak_self.clone(), index, section, &tracks).map(Arc::new) {
                    Ok(section) => sections.push(section),
                    Err(err) => println!(
                        "Tab::try_parse_arc(), bad setion: {} {} -> {}",
                        index, section_id, err
                    ),
                }
            }
            let form = Form::from((proto.form, &sections));
            let bars = Self::new_tab_bars(weak_self, &meta, &form);
            Self {
                uuid,
                meta,
                tracks,
                sections,
                form,
                bars,
            }
        })
    }
    fn new_tab_bars(weak_self: &Weak<Tab>, meta: &TabMeta, form: &Form) -> Vec<Arc<TabBar>> {
        let mut section_rounds: HashMap<String, usize> = HashMap::new();
        let mut section_ordinal: usize = 1;
        let mut bar_ordinal: usize = 1;
        let mut bars: Vec<Arc<TabBar>> = vec![];
        for section in form.sections.iter() {
            let section_round = match section_rounds.get(&section.id) {
                Some(r) => r + 1,
                None => 1,
            };
            section_rounds.insert(section.id.clone(), section_round);
            bars.extend(section.new_tab_bars(
                section.clone(),
                weak_self.clone(),
                section_round,
                section_ordinal,
                bar_ordinal,
                meta.bar_units(),
            ));
            section_ordinal += 1;
            bar_ordinal += section.bars.len();
        }
        println!("new_tab_bars() -> {:?} bars", bars.len());
        bars
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
        bar_units: Units,
    ) -> Vec<Arc<TabBar>> {
        self.bars
            .iter()
            .enumerate()
            .map(|(bar_index, bar)| {
                TabBar::new_arc(
                    tab.clone(),
                    arc_section.clone(),
                    bar.clone(),
                    section_round,
                    section_ordinal,
                    bar_index,
                    section_bar_ordinal + bar_index,
                    bar_units,
                )
            })
            .collect()
    }
}
impl ModelEntry {
    pub fn calc_tied_units(entries: &Vec<ProtoEntry>, index: usize) -> Units {
        let mut units = Units(0.0);
        if let Some(entry) = entries.get(index) {
            units = units + Units::from(entry.duration());
            if let Some(next_entry) = entries.get(index + 1) {
                if next_entry.is_core_tie() {
                    for i in index + 2..entries.len() {
                        let peek_entry = entries.get(i).unwrap();
                        if peek_entry.duration() != Duration::Zero {
                            return units + Self::calc_tied_units(entries, i);
                        }
                    }
                }
            }
        }
        units
    }
    pub fn new_entries(v: Vec<ProtoEntry>, track: &Weak<Track>) -> Vec<Arc<ModelEntry>> {
        let entries = v.clone();
        v.into_iter()
            .map(Arc::new)
            .enumerate()
            .map(|(index, entry)| {
                let tied_units = Self::calc_tied_units(&entries, index);
                ModelEntry::new(track.clone(), entry, index, tied_units)
            })
            .map(Arc::new)
            .collect()
    }
}
impl LaneEntry {
    pub fn new_entries(v: Vec<Arc<ModelEntry>>, lane: &Weak<BarLane>) -> Vec<Arc<LaneEntry>> {
        let mut pos = 0.0;
        v.into_iter()
            .enumerate()
            .map(|(index, entry)| {
                let in_bar_pos = pos;
                pos += Units::from(entry.as_ref().duration()).0;
                LaneEntry::new(lane.clone(), index, entry, Units(in_bar_pos))
            })
            .map(Arc::new)
            .collect()
    }
}
