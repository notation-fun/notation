use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::BevyUtil;
use notation_model::prelude::{Note, PlayingState, Syllable, SyllableNote};

use crate::prelude::{EntryData, LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

use super::tone_mode::ToneMode;

pub type ToneNoteData = EntryData<ToneNoteValue>;

#[derive(Clone, Debug)]
pub struct ToneNoteValue {
    pub note: Note,
    pub mode: ToneMode,
    pub syllable_note: SyllableNote,
    pub playing_state: PlayingState,
    pub bar_size: f32,
}

impl ToneNoteValue {
    pub fn new(tab_bar: &TabBar, note: Note, mode: ToneMode) -> Self {
        let syllable_note = tab_bar.calc_syllable_note(&note);
        Self {
            note,
            mode,
            syllable_note,
            playing_state: PlayingState::Idle,
            bar_size: 0.0,
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

impl<'a> ToneNoteShape<'a> {
    fn calc_width_height(&self) -> (f32, f32) {
        let outline = self
            .theme
            .sizes
            .melody
            .note_outline
            .of_state(&self.data.value.playing_state);
        let mut width = self.data.value.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.tied_units.0;
        let mut height = self.theme.melody.note_height;
        if self.data.value.playing_state.is_current() {
            height += outline;
        } else {
            width -= outline * 2.0;
        }
        (width, height)
    }
}

impl<'a> LyonShape<shapes::Rectangle> for ToneNoteShape<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{}",
            self.data.bar_props.bar_ordinal, self.data.value.note
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
            self.theme.colors.of_syllable(self.data.value.syllable()),
            self.theme
                .colors
                .syllables
                .outline
                .of_state(&self.data.value.playing_state),
        )
    }
    fn get_draw_mode(&self) -> DrawMode {
        let outline = self
            .theme
            .sizes
            .melody
            .note_outline
            .of_state(&self.data.value.playing_state);
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(outline),
        }
    }
    fn get_transform(&self) -> Transform {
        if self.data.value.bar_size <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let x = self.data.value.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.in_bar_pos.0;
        let y = if self.data.value.mode.is_melody() {
            self.theme
                .melody
                .calc_note_y(self.data.value.note, self.data.value.syllable_note)
        } else {
            0.0
        };
        let (width, height) = self.calc_width_height();
        Transform::from_xyz(x + width / 2.0, y + height / 2.0, self.theme.strings.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, ToneNoteData, shapes::Rectangle, ToneNoteShape<'a>>
    for ToneNoteShape<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: ToneNoteData) -> ToneNoteShape<'a> {
        ToneNoteShape::<'a> { theme, data }
    }
}
