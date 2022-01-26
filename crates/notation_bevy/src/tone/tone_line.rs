use bevy::prelude::*;
use notation_bevy_utils::prelude::{ShapeOp, StrokeLine};

use crate::prelude::{LaneData, NotationTheme, Note};

use super::tone_mode::ToneMode;

#[derive(Clone, Debug)]
pub struct ToneLineValue {
    pub mode: ToneMode,
    pub is_root: bool,
    pub note: Note,
    pub bar_size: f32,
}

pub type ToneLineData = LaneData<ToneLineValue>;

impl ShapeOp<NotationTheme, StrokeLine> for ToneLineData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeLine {
        let sizes = match self.value.mode {
            ToneMode::Melody => theme.sizes.melody,
            _ => theme.sizes.harmony,
        };
        let y = sizes.calc_note_y(self.value.note);
        let color = theme.colors.bar.grid_color;
        let line_width = if self.value.is_root {
            theme.sizes.bar.grid_root_line_width
        } else {
            theme.sizes.bar.grid_line_width
        };
        StrokeLine {
            from: Vec2::ZERO,
            to: Vec2::new(self.value.bar_size, 0.0),
            line_width,
            color,
            offset: Vec3::new(0.0, y, theme.z.grid),
        }
    }
}
