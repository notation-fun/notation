use std::fmt::Display;

use bevy::prelude::*;
use bevy_utils::prelude::{BevyUtil, ShapeOp, StrokeLine};

use crate::prelude::{BarData, NotationTheme};

#[derive(Clone, Debug)]
pub struct MiniSectionSeparatorValue {
    pub width: f32,
    pub x_offset: f32,
}
impl MiniSectionSeparatorValue {
    pub fn new(x_offset: f32) -> Self {
        Self {
            width: 0.0,
            x_offset,
        }
    }
}
impl Display for MiniSectionSeparatorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub type MiniSectionSeparatorData = BarData<MiniSectionSeparatorValue>;

impl ShapeOp<NotationTheme, StrokeLine> for MiniSectionSeparatorData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeLine {
        let offset = if self.value.width <= 0.0 {
            BevyUtil::offscreen_offset()
        } else {
            let line_width = theme.sizes.mini_map.section_separator;
            let x_offset = -self.value.width / 2.0;
            Vec3::new(line_width + x_offset, 0.0, theme.core.mini_bar_z + 2.0)
        };
        StrokeLine {
            from: Vec2::new(
                0.0,
                theme.sizes.mini_map.bar_height / 2.0
                    + theme.sizes.mini_map.bar_margin().height * 2.0,
            ),
            to: Vec2::new(0.0, -theme.sizes.mini_map.bar_height / 2.0),
            line_width: theme.sizes.mini_map.section_separator,
            color: theme
                .colors
                .of_section(self.bar_props.section_index),
            offset,
        }
    }
}