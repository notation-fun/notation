use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{BarPosition, Duration, Syllable, Units};

use crate::config::bevy_config::BevyConfig;
use crate::prelude::{LyonShape, LyonShapeOp};
use notation_model::prelude::TabBar;

#[derive(Clone, Debug)]
pub struct PickNoteData {
    pub bar_ordinal: usize,
    pub duration: Duration,
    pub position: BarPosition,
    pub string: u8,
    pub syllable: Syllable,
}

impl PickNoteData {
    pub fn new(
        tab_bar: &Arc<TabBar>,
        duration: Duration,
        position: BarPosition,
        string: u8,
        syllable: Syllable,
    ) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        PickNoteData {
            bar_ordinal,
            duration,
            position,
            string,
            syllable,
        }
    }
}
pub struct PickNote<'a> {
    config: &'a BevyConfig,
    data: PickNoteData,
}

impl<'a> LyonShape<shapes::Rectangle> for PickNote<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{} String {}",
            self.data.bar_ordinal, self.data.syllable, self.data.string
        )
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.config.grid.unit_size * Units::from(self.data.duration).0,
            height: self.config.grid.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(
            self.config
                .theme
                .syllable
                .color_of_syllable(self.data.syllable),
        )
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default()
                .with_line_width(self.config.grid.note_outline),
        }
    }
    fn get_transform(&self) -> Transform {
        let x = self.config.grid.unit_size * self.data.position.in_bar_pos.0;
        let y = -1.0 * self.config.theme.fretted.string_space * self.data.string as f32
            - self.config.grid.note_height / 2.0;
        Transform::from_xyz(x, y, self.config.theme.fretted.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, PickNoteData, shapes::Rectangle, PickNote<'a>> for PickNote<'a> {
    fn new_shape(config: &'a BevyConfig, data: PickNoteData) -> PickNote<'a> {
        PickNote::<'a> { config, data }
    }
}
