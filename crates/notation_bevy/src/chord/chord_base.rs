use bevy::prelude::*;
use notation_model::prelude::{Interval, ModelEntryProps, Syllable};

use crate::prelude::NotationTheme;

use super::chord_note::{ChordNote, ChordNoteData, ChordNoteExtra, ChordNoteValue};

#[derive(Clone, Debug)]
pub struct ChordBaseExtra {
    pub diagram_radius: f32,
}

pub type ChordBaseData = ChordNoteData<ChordBaseExtra>;
pub type ChordBase<'a> = ChordNote<'a, ChordBaseExtra>;

impl ChordNoteExtra for ChordBaseExtra {
    fn set_diagram_radius(&mut self, diagram_radius: f32) {
        self.diagram_radius = diagram_radius;
    }
    fn radius(&self, theme: &NotationTheme) -> f32 {
        self.diagram_radius * theme.sizes.chord.diagram_base_factor
    }
    fn offset(&self, theme: &NotationTheme) -> Vec2 {
        let y = self.radius(theme) * theme.sizes.chord.diagram_base_y_factor;
        Vec2::new(0.0, -y)
    }
}

impl ChordBaseData {
    pub fn new_data(
        entry_props: ModelEntryProps,
        root: Syllable,
        interval: Interval,
        diagram_radius: f32,
    ) -> Self {
        let extra = ChordBaseExtra { diagram_radius };
        Self::from((
            entry_props,
            ChordNoteValue::<ChordBaseExtra>::new(root, interval, extra),
        ))
    }
}
