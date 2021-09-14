use std::fmt::Display;
use std::sync::{Arc, Weak};

use notation_proto::prelude::{
    Chord, Fretboard4, Fretboard6, HandShape4, HandShape6, Note, SyllableNote, TabPosition,
};

use crate::prelude::{
    Bar, BarLane, LaneEntry, LaneKind, Pitch, Section, Signature, Syllable, Tab, TabMeta, Unit,
    Units,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct TabBarProps {
    pub section_index: usize,
    pub section_round: usize,
    pub section_ordinal: usize,
    pub bar_index: usize,
    pub bar_ordinal: usize,
    pub bar_units: Units,
}

#[derive(Debug)]
pub struct TabBar {
    pub tab: Weak<Tab>,
    pub section: Arc<Section>,
    pub lanes: Vec<Arc<BarLane>>,
    pub model: Arc<Bar>,
    pub props: TabBarProps,
}
impl Display for TabBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<TabBar>({} S:{} {}:{} N:{})",
            self.props.bar_ordinal,
            self.props.section_ordinal,
            self.props.section_index,
            self.props.bar_index,
            self.lanes.len(),
        )
    }
}
impl TabBar {
    pub fn new_arc(
        tab: Weak<Tab>,
        section: Arc<Section>,
        bar: Arc<Bar>,
        section_round: usize,
        section_ordinal: usize,
        bar_index: usize,
        bar_ordinal: usize,
        bar_units: Units,
    ) -> Arc<Self> {
        Arc::<Self>::new_cyclic(|weak_self| {
            let mut lanes = Vec::new();
            let mut index = 0;
            for layer in bar.layers.iter() {
                for slice in layer.slices.iter() {
                    if slice.in_round(section_round) {
                        if let Some(lane) = BarLane::try_new_arc(
                            weak_self.clone(),
                            index,
                            &layer.track,
                            slice.clone(),
                        ) {
                            lanes.push(lane);
                            index += 1;
                        }
                    }
                }
            }
            let props = TabBarProps {
                section_index: section.index,
                section_round,
                section_ordinal,
                bar_index,
                bar_ordinal,
                bar_units,
            };
            Self {
                tab: tab,
                section: section,
                model: bar,
                lanes,
                props,
            }
        })
    }
    pub fn tab_position(&self) -> TabPosition {
        TabPosition::new(Units(
            (self.props.bar_ordinal - 1) as f32 * self.bar_units().0,
        ))
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
impl TabBar {
    pub fn tab(&self) -> Option<Arc<Tab>> {
        self.tab.upgrade().map(|x| x.clone())
    }
    pub fn get_lane_of_kind(
        &self,
        kind: LaneKind,
        track_index: Option<usize>,
    ) -> Option<Arc<BarLane>> {
        for lane in self.lanes.iter() {
            if lane.kind == kind {
                if track_index.is_none() || track_index.unwrap() == lane.track.props.index {
                    return Some(lane.clone());
                }
            }
        }
        None
    }
    pub fn get_entry_in_other_lane<T, F: Fn(&LaneEntry) -> Option<T>>(
        &self,
        lane_kind: LaneKind,
        track_index: Option<usize>,
        in_bar_pos: Option<Units>,
        predicate: &F,
    ) -> Option<T> {
        if let Some(lane) = self.get_lane_of_kind(lane_kind, track_index) {
            for entry in lane.entries.iter() {
                let mut in_range = in_bar_pos.is_none();
                if let Some(in_bar_pos) = in_bar_pos {
                    if in_bar_pos < entry.props.in_bar_pos {
                        break;
                    } else {
                        in_range = in_bar_pos < entry.props.in_bar_pos + entry.model().props.tied_units
                    }
                }
                if in_range {
                    if let Some(result) = predicate(entry.as_ref()) {
                        return Some(result);
                    }
                }
            }
        }
        None
    }
    pub fn get_chords(&self) -> Vec<Chord> {
        let mut chords = Vec::new();
        if let Some(lane) = self.get_lane_of_kind(LaneKind::Chord, None) {
            for entry in lane.entries.iter() {
                if let Some(chord) = entry.proto().as_core().and_then(|x| x.as_chord()) {
                    chords.push(chord.clone());
                }
            }
        }
        chords
    }
    pub fn get_chord(&self, in_bar_pos: Option<Units>) -> Option<Chord> {
        self.get_entry_in_other_lane(LaneKind::Chord, None, in_bar_pos, &|x: &LaneEntry| {
            x.proto()
                .as_core()
                .and_then(|x| x.as_chord())
                .map(|z| z.to_owned())
        })
    }
    pub fn get_chord_of_entry(&self, entry: &LaneEntry) -> Option<Chord> {
        self.get_chord(Some(entry.props.in_bar_pos))
    }
}

macro_rules! impl_get_fretted_shape {
    ($name:ident, $strings:literal, $as_fretted:ident, $get_fretboard:ident, $fretboard:ident, $hand_shape:ident) => {
        impl TabBar {
            pub fn $name(&self, entry: &LaneEntry) -> Option<($fretboard, $hand_shape)> {
                self.get_entry_in_other_lane(
                    LaneKind::Shapes,
                    Some(entry.lane_props().track.index),
                    Some(entry.props.in_bar_pos),
                    &|x: &LaneEntry| {
                        x.model()
                            .$as_fretted()
                            .and_then(|y| y.as_shape())
                            .and_then(|z| {
                                x.lane()
                                    .and_then(|lane| lane.track.$get_fretboard())
                                    .map(|fretboard| (fretboard, z.clone()))
                            })
                    },
                )
            }
        }
    };
}

impl_get_fretted_shape!(
    get_fretted_shape6,
    6,
    as_fretted6,
    get_fretboard6,
    Fretboard6,
    HandShape6
);
impl_get_fretted_shape!(
    get_fretted_shape4,
    4,
    as_fretted4,
    get_fretboard4,
    Fretboard4,
    HandShape4
);
