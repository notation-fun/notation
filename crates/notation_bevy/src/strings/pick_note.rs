use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{PickNote, Syllable};

use crate::prelude::{EntryData, LyonShape, LyonShapeOp, NotationTheme};

pub type PickNoteData = EntryData<PickNoteValue>;

#[derive(Clone, Debug)]
pub struct PickNoteValue {
    pub pick_note: PickNote,
    pub syllable: Syllable,
}

impl PickNoteValue {
    pub fn new(pick_note: PickNote, syllable: Syllable) -> Self {
        Self {
            pick_note,
            syllable,
        }
    }
}
pub struct PickNoteShape<'a> {
    theme: &'a NotationTheme,
    data: PickNoteData,
}

impl<'a> LyonShape<shapes::Rectangle> for PickNoteShape<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{} {}",
            self.data.bar_props.bar_ordinal, self.data.value.syllable, self.data.value.pick_note
        )
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.theme.grid.bar_size / self.data.bar_props.bar_units.0
                * self.data.entry_props.tied_units.0
                - self.theme.strings.note_outline * 2.0,
            height: self.theme.strings.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::outlined(
            self.theme
                .colors
                .of_syllable(self.data.value.syllable),
            self.theme.strings.note_outline_color,
        )
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default()
                .with_line_width(self.theme.strings.note_outline),
        }
    }
    fn get_transform(&self) -> Transform {
        let x = self.theme.grid.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.in_bar_pos.0;
        let y = -1.0
            * self.theme.strings.string_space
            * (self.data.value.pick_note.string as f32 - 0.5)
            - self.theme.strings.note_height / 2.0;
        Transform::from_xyz(x, y, self.theme.strings.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, PickNoteData, shapes::Rectangle, PickNoteShape<'a>> for PickNoteShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: PickNoteData) -> PickNoteShape<'a> {
        PickNoteShape::<'a> { theme, data }
    }
}
