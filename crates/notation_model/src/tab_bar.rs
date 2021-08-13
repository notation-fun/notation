use std::fmt::Display;
use std::sync::{Arc, Weak};

use notation_proto::prelude::{
    Fretboard4, Fretboard6, HandShape4, HandShape6, Note, SyllableNote, TabPosition,
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
            for layer in bar.layers.iter() {
                for slice in layer.slices.iter() {
                    if slice.in_round(section_round) {
                        if let Some(lane) =
                            BarLane::try_new_arc(weak_self.clone(), &layer.track, slice.clone())
                        {
                            lanes.push(lane);
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
    pub fn tab_pos(&self) -> TabPosition {
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
    pub fn get_lane_of_kind(&self, kind: LaneKind) -> Option<Arc<BarLane>> {
        for lane in self.lanes.iter() {
            if lane.kind == kind {
                return Some(lane.clone());
            }
        }
        None
    }
}

macro_rules! impl_get_fretted_shape {
    ($name:ident, $strings:literal, $as_fretted:ident, $get_fretboard:ident, $fretboard:ident, $hand_shape:ident) => {
        impl TabBar {
            pub fn $name(&self, entry: &LaneEntry) -> Option<($fretboard, $hand_shape)> {
                if let Some(shapes_lane) = self.get_lane_of_kind(LaneKind::Shapes) {
                    for lane_entry in shapes_lane.entries.iter() {
                        if entry.props.in_bar_pos
                            > lane_entry.props.in_bar_pos + lane_entry.model().props.tied_units
                        {
                            continue;
                        }
                        if entry.props.in_bar_pos < lane_entry.props.in_bar_pos {
                            break;
                        }
                        if let Some(fretted_entry) = lane_entry.model().$as_fretted() {
                            if let Some((shape, _duration)) = fretted_entry.as_shape() {
                                if let Some(fretboard) = shapes_lane.track.$get_fretboard() {
                                    return Some((fretboard, shape.clone()));
                                } else {
                                    return None;
                                }
                            }
                        }
                    }
                }
                None
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
