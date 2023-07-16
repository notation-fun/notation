use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::prelude::{ShapeOp, StrokeLine};

use crate::prelude::{LaneData, NotationTheme};

#[derive(Clone, Debug)]
pub struct SingleStringValue {
    pub string: u8,
    pub bar_size: f32,
}

pub type SingleStringData = LaneData<SingleStringValue>;

impl ShapeOp<NotationTheme, StrokeLine> for SingleStringData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeLine {
        let y = theme.sizes.strings.calc_string_y(self.value.string);
        StrokeLine {
            from: Vec2::ZERO,
            to: Vec2::new(self.value.bar_size, 0.0),
            line_width: theme.guitar.get_string_width(self.value.string),
            color: theme.colors.strings.string.idle,
            offset: Vec3::new(0.0, y, theme.z.string),
        }
    }
}
