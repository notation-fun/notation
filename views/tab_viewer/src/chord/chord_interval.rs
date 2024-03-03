use std::f32::consts::PI;

use edger_bevy::bevy_prelude::*;
use notation_model::prelude::{Interval, ModelEntryProps, Syllable};

use crate::prelude::NotationTheme;

use super::chord_note::{ChordNoteData, ChordNoteExtra, ChordNoteValue};

#[derive(Clone, Debug)]
pub struct ChordIntervalExtra {
    pub total: usize,
    pub index: usize,
    pub diagram_radius: f32,
}
pub type ChordIntervalData = ChordNoteData<ChordIntervalExtra>;

impl ChordNoteExtra for ChordIntervalExtra {
    fn set_diagram_radius(&mut self, diagram_radius: f32) {
        self.diagram_radius = diagram_radius;
    }
    fn radius(&self, theme: &NotationTheme) -> f32 {
        self.diagram_radius * theme.sizes.chord.diagram_interval_radius_factor
    }
    fn offset(&self, theme: &NotationTheme) -> Vec2 {
        let angle_offset = match self.total {
            2 => -180,
            3 => -150,
            4 => -135,
            _ => 0,
        } as f32
            * PI
            / 180.0;
        let angle = PI * 2.0 * self.index as f32 / self.total as f32 + angle_offset;
        let factor = if self.total == 1 {
            0.0
        } else {
            theme.sizes.chord.diagram_interval_offset_factor
        };
        Vec2::new(
            self.diagram_radius * factor * angle.cos(),
            self.diagram_radius * factor * angle.sin(),
        )
    }
}

impl ChordIntervalData {
    pub fn new_data(
        entry_props: ModelEntryProps,
        root: Syllable,
        interval: Interval,
        total: usize,
        index: usize,
        diagram_radius: f32,
    ) -> Self {
        let extra = ChordIntervalExtra {
            total,
            index,
            diagram_radius,
        };
        Self::from((
            entry_props,
            ChordNoteValue::<ChordIntervalExtra>::new(root, interval, extra),
        ))
    }
}
