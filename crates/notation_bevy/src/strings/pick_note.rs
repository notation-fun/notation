use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{BarPosition, Duration, PickNote, Syllable, Units};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

#[derive(Clone, Debug)]
pub struct PickNoteData {
    pub bar_units: Units,
    pub bar_ordinal: usize,
    pub duration: Duration,
    pub position: BarPosition,
    pub pick_note: PickNote,
    pub syllable: Syllable,
}

impl PickNoteData {
    pub fn new(
        bar_units: Units,
        tab_bar: &Arc<TabBar>,
        duration: Duration,
        position: BarPosition,
        pick_note: PickNote,
        syllable: Syllable,
    ) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        PickNoteData {
            bar_units,
            bar_ordinal,
            duration,
            position,
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
            self.data.bar_ordinal, self.data.syllable, self.data.pick_note
        )
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.theme.grid.bar_size / self.data.bar_units.0
                * Units::from(self.data.duration).0
                - self.theme.strings.note_outline * 2.0,
            height: self.theme.strings.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::outlined(
            self.theme.syllable.color_of_syllable(self.data.syllable),
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
        let x = self.theme.grid.bar_size / self.data.bar_units.0 * self.data.position.in_bar_pos.0;
        let y = -1.0 * self.theme.strings.string_space * (self.data.pick_note.string as f32 - 0.5)
            - self.theme.strings.note_height / 2.0;
        Transform::from_xyz(x, y, self.theme.strings.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, PickNoteData, shapes::Rectangle, PickNoteShape<'a>> for PickNoteShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: PickNoteData) -> PickNoteShape<'a> {
        PickNoteShape::<'a> { theme, data }
    }
}