use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{PickNote, PlayingState, Syllable};

use crate::prelude::{EntryData, LyonShape, LyonShapeOp, NotationTheme};

pub type PickNoteData = EntryData<PickNoteValue>;

#[derive(Clone, Debug)]
pub struct PickNoteValue {
    pub pick_note: PickNote,
    pub syllable: Syllable,
    pub playing_state: PlayingState,
}

impl PickNoteValue {
    pub fn new(pick_note: PickNote, syllable: Syllable) -> Self {
        Self {
            pick_note,
            syllable,
            playing_state: PlayingState::Idle,
        }
    }
}

impl PickNoteData {
    pub fn calc_width_height(&self, theme: &NotationTheme) -> (f32, f32) {
        let outline = theme
            .sizes
            .strings
            .note_outline
            .of_state(&self.value.playing_state);
        let mut width =
            theme.grid.bar_size / self.bar_props.bar_units.0 * self.entry_props.tied_units.0;
        let mut height = theme.sizes.strings.note_height;
        if self.value.playing_state.is_current() {
            height += outline;
        } else {
            width -= outline * 2.0;
        }
        (width, height)
    }
}
pub struct PickNoteShape<'a> {
    theme: &'a NotationTheme,
    data: PickNoteData,
}

impl<'a> PickNoteShape<'a> {
    fn calc_width_height(&self) -> (f32, f32) {
        self.data.calc_width_height(self.theme)
    }
}

impl<'a> LyonShape<shapes::Rectangle> for PickNoteShape<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{} {}",
            self.data.bar_props.bar_ordinal, self.data.value.syllable, self.data.value.pick_note
        )
    }
    fn get_shape(&self) -> shapes::Rectangle {
        let (width, height) = self.calc_width_height();
        shapes::Rectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::Center,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::outlined(
            self.theme.colors.of_syllable(self.data.value.syllable),
            self.theme
                .colors
                .strings
                .outline
                .of_state(&self.data.value.playing_state),
        )
    }
    fn get_draw_mode(&self) -> DrawMode {
        let outline = self
            .theme
            .sizes
            .strings
            .note_outline
            .of_state(&self.data.value.playing_state);
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(outline),
        }
    }
    fn get_transform(&self) -> Transform {
        let x = self.theme.grid.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.in_bar_pos.0;
        let y = -1.0
            * self.theme.strings.string_space
            * (self.data.value.pick_note.string as f32 - 0.5)
            - self.theme.strings.note_height / 2.0;
        let (width, height) = self.calc_width_height();
        Transform::from_xyz(x + width / 2.0, y + height / 2.0, self.theme.strings.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, PickNoteData, shapes::Rectangle, PickNoteShape<'a>>
    for PickNoteShape<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: PickNoteData) -> PickNoteShape<'a> {
        PickNoteShape::<'a> { theme, data }
    }
}
