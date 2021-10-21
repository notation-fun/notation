use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{ShapeOp, StrokeLine};

use crate::prelude::{LaneData, NotationTheme};

#[derive(Clone, Debug)]
pub struct SingleStringValue {
    pub string: u8,
    pub bar_size: f32,
}

pub type SingleStringData = LaneData<SingleStringValue>;

impl ShapeOp<NotationTheme, shapes::Line, StrokeLine> for SingleStringData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeLine {
        let y = -1.0 * (self.value.string as f32 - 1.0) * theme.sizes.strings.string_space;
        StrokeLine {
            from: Vec2::ZERO,
            to: Vec2::new(self.value.bar_size, 0.0),
            line_width: theme.guitar.get_string_width(self.value.string),
            color: theme.strings.string_color,
            offset: Vec3::new(0.0, y, theme.strings.string_z),
        }
    }
}
