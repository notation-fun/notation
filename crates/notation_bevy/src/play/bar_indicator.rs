use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, LayoutData, StrokeRectangle, ShapeOp};
use notation_model::prelude::TabBarProps;

use crate::prelude::{NotationTheme};

#[derive(Clone, Debug)]
pub struct BarIndicatorData {
    pub bar_props: TabBarProps,
    pub bar_layout: LayoutData,
}

impl BarIndicatorData {
    pub fn new() -> Self {
        BarIndicatorData {
            bar_props: TabBarProps::default(),
            bar_layout: LayoutData::ZERO,
        }
    }
}

impl ShapeOp<NotationTheme, StrokeRectangle> for BarIndicatorData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeRectangle {
        let offset = if self.bar_layout.size.width <= 0.0 {
            BevyUtil::offscreen_offset()
        } else {
            let x = self.bar_layout.offset.x;
            let y = self.bar_layout.offset.y + theme.sizes.bar.bar_separator_extra;
            Vec3::new(x, y, theme.core.bar_indicator_z)
        };
        StrokeRectangle {
            width: self.bar_layout.size.width,
            height: self.bar_layout.size.height + theme.sizes.bar.bar_separator_extra * 2.0,
            origin: shapes::RectangleOrigin::TopLeft,
            color: theme.colors.bar.bar_indicator,
            line_width: theme.sizes.bar.pos_indicator_size,
            offset,
        }
    }
}