use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{
    BarPosition, Duration, Note, Syllable, SyllableNote, Units,
};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

use super::tone_mode::ToneMode;

#[derive(Clone, Debug)]
pub struct ToneNoteData {
    pub bar_units: Units,
    pub bar_ordinal: usize,
    pub duration: Duration,
    pub tied_units: Units,
    pub position: BarPosition,
    pub note: Note,
    pub mode: ToneMode,
    pub syllable_note: SyllableNote,
}

impl ToneNoteData {
    pub fn new(
        bar_units: Units,
        tab_bar: &Arc<TabBar>,
        duration: Duration,
        tied_units: Units,
        position: BarPosition,
        note: Note,
        mode: ToneMode,
    ) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        let syllable_note = tab_bar.calc_syllable_note(&note);
        ToneNoteData {
            bar_units,
            bar_ordinal,
            duration,
            tied_units,
            position,
            note,
            mode,
            syllable_note,
        }
    }
    pub fn syllable(&self) -> Syllable {
        self.syllable_note.syllable
    }
    pub fn units(&self) -> Units {
        //Units::from(self.duration)
        self.tied_units
    }
}
pub struct ToneNoteShape<'a> {
    theme: &'a NotationTheme,
    data: ToneNoteData,
}

impl<'a> LyonShape<shapes::Rectangle> for ToneNoteShape<'a> {
    fn get_name(&self) -> String {
        format!("{}:{}", self.data.bar_ordinal, self.data.note)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.theme.grid.bar_size / self.data.bar_units.0
                * self.data.units().0
                - self.theme.melody.note_outline * 2.0,
            height: self.theme.melody.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::outlined(
            self.theme.syllable.color_of_syllable(self.data.syllable()),
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
        let x = self.theme.grid.bar_size / self.data.bar_units.0 * self.data.position.in_bar_pos.0;
        let y = if self.data.mode.is_melody() {
            self.theme
                .melody
                .calc_note_y(self.data.note, self.data.syllable_note)
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
