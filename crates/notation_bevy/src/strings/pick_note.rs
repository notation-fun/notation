use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, OutlineRectangle, ShapeOp};
use notation_model::prelude::{PickNote, PlayingState, Syllable};

use crate::prelude::{EntryData, NotationTheme};

pub type PickNoteData = EntryData<PickNoteValue>;

#[derive(Clone, Debug)]
pub struct PickNoteValue {
    pub pick_note: PickNote,
    pub syllable: Syllable,
    pub playing_state: PlayingState,
    pub bar_size: f32,
}

impl PickNoteValue {
    pub fn new(pick_note: PickNote, syllable: Syllable) -> Self {
        Self {
            pick_note,
            syllable,
            playing_state: PlayingState::Idle,
            bar_size: 0.0,
        }
    }
}

impl PickNoteData {
    pub fn calc_outline(&self, theme: &NotationTheme) -> f32 {
        theme
            .sizes
            .strings
            .note_outline
            .of_state(&self.value.playing_state)
    }
    pub fn calc_width_height(&self, theme: &NotationTheme) -> (f32, f32) {
        let width =
            self.value.bar_size / self.bar_props.bar_units.0 * self.entry_props.tied_units.0;
        let mut height = theme.sizes.strings.note_height;
        let outline = self.calc_outline(theme);
        if self.value.playing_state.is_current() {
            height += outline * 2.0;
        }
        (width - outline * 2.0, height)
    }
    pub fn calc_outline_color(&self, theme: &NotationTheme) -> Color {
        theme
            .colors
            .strings
            .outline
            .of_state(&self.value.playing_state)
    }
    pub fn calc_fret_color(&self, theme: &NotationTheme) -> Color {
        theme
            .colors
            .strings
            .fret
            .of_state(&self.value.playing_state)
    }
}

impl ShapeOp<NotationTheme, OutlineRectangle> for PickNoteData {
    fn get_shape(&self, theme: &NotationTheme) -> OutlineRectangle {
        let (width, height) = self.calc_width_height(theme);
        let color = theme.colors.of_syllable(self.value.syllable);
        let outline_color = self.calc_outline_color(theme);
        let outline_width = self.calc_outline(theme);
        let offset = if self.value.bar_size <= 0.0 {
            BevyUtil::offscreen_offset()
        } else {
            let x = self.value.bar_size / self.bar_props.bar_units.0
                * self.entry_props.in_bar_pos.0;
            let y = -1.0
                * theme.sizes.strings.string_space
                * (self.value.pick_note.string as f32 - 1.0);
            let extra_z = if self.value.playing_state.is_current() {
                1.0
            } else {
                0.0
            };
            Vec3::new(x, y + height / 2.0, theme.strings.pick_z + extra_z)
        };
        OutlineRectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::TopLeft,
            color,
            outline_width,
            outline_color,
            offset,
        }
    }
}