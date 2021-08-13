use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{Note, Syllable, SyllableNote};

use crate::prelude::{EntryData, LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

use super::tone_mode::ToneMode;

pub type ToneNoteData = EntryData<ToneNoteValue>;

#[derive(Clone, Debug)]
pub struct ToneNoteValue {
    pub note: Note,
    pub mode: ToneMode,
    pub syllable_note: SyllableNote,
}

impl ToneNoteValue {
    pub fn new(tab_bar: &TabBar, note: Note, mode: ToneMode) -> Self {
        let syllable_note = tab_bar.calc_syllable_note(&note);
        Self {
            note,
            mode,
            syllable_note,
        }
    }
    pub fn syllable(&self) -> Syllable {
        self.syllable_note.syllable
    }
}
pub struct ToneNoteShape<'a> {
    theme: &'a NotationTheme,
    data: ToneNoteData,
}

impl<'a> LyonShape<shapes::Rectangle> for ToneNoteShape<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{}",
            self.data.bar_props.bar_ordinal, self.data.value.note
        )
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.theme.grid.bar_size / self.data.bar_props.bar_units.0
                * self.data.entry_props.tied_units.0
                - self.theme.melody.note_outline * 2.0,
            height: self.theme.melody.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::outlined(
            self.theme
                .colors
                .color_of_syllable(self.data.value.syllable()),
            self.theme.melody.note_outline_color,
        )
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default()
                .with_line_width(self.theme.melody.note_outline),
        }
    }
    fn get_transform(&self) -> Transform {
        let x = self.theme.grid.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.in_bar_pos.0;
        let y = if self.data.value.mode.is_melody() {
            self.theme
                .melody
                .calc_note_y(self.data.value.note, self.data.value.syllable_note)
        } else {
            0.0
        };
        Transform::from_xyz(x, y, self.theme.strings.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, ToneNoteData, shapes::Rectangle, ToneNoteShape<'a>> for ToneNoteShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: ToneNoteData) -> ToneNoteShape<'a> {
        ToneNoteShape::<'a> { theme, data }
    }
}
