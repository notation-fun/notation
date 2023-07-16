use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::prelude::{offscreen, LayoutSize, ShapeOp, StrokeLine};

use crate::prelude::{BarData, NotationTheme};

#[derive(Clone, Debug)]
pub struct BarSeparatorValue {
    pub is_begin: bool,
    pub bar_size: LayoutSize,
}
pub type BarSeparatorData = BarData<BarSeparatorValue>;

impl BarSeparatorValue {
    pub fn new(is_begin: bool) -> Self {
        Self {
            is_begin,
            bar_size: LayoutSize::ZERO,
        }
    }
}

impl ShapeOp<NotationTheme, StrokeLine> for BarSeparatorData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeLine {
        let offset = if self.value.bar_size.width <= 0.0 {
            offscreen::offset()
        } else {
            let x = if self.value.is_begin {
                0.0
            } else {
                self.value.bar_size.width
            };
            Vec3::new(x, 0.0, theme.z.bar_separator)
        };
        StrokeLine {
            from: Vec2::new(0.0, theme.sizes.bar.bar_separator_extra),
            to: Vec2::new(
                0.0,
                -self.value.bar_size.height - theme.sizes.bar.bar_separator_extra,
            ),
            line_width: theme.sizes.bar.bar_separator_size,
            color: theme.colors.bar.bar_separator_color,
            offset,
        }
    }
}
