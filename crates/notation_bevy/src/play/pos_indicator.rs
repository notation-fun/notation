use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, LayoutData, OutlineRectangle, ShapeOp};
use notation_model::prelude::{BarPosition, Position, TabBarProps, Units};

use crate::prelude::{NotationTheme};

#[derive(Clone, Debug)]
pub struct PosIndicatorData {
    pub bar_props: TabBarProps,
    pub bar_layout: LayoutData,
    pub bar_units: Units,
    pub bar_position: BarPosition,
}

impl PosIndicatorData {
    pub fn new(bar_units: Units) -> Self {
        PosIndicatorData {
            bar_props: TabBarProps::default(),
            bar_layout: LayoutData::ZERO,
            bar_units,
            bar_position: BarPosition::ZERO,
        }
    }
    pub fn is_synced(&self) -> bool {
        self.bar_position.bar_ordinal == self.bar_props.bar_ordinal
    }
    pub fn offset_x(&self) -> f32 {
        let mut x = self.bar_layout.offset.x;
        x += self.bar_layout.size.width * self.bar_position.in_bar_pos.0 / self.bar_units.0;
        x
    }
}

impl ShapeOp<NotationTheme, OutlineRectangle> for PosIndicatorData {
    fn get_shape(&self, theme: &NotationTheme) -> OutlineRectangle {
        let width = theme.sizes.bar.pos_indicator_size;
        let height = self.bar_layout.size.height + theme.sizes.bar.bar_separator_extra * 2.0;
        let color = theme
                .colors
                .of_section(self.bar_props.section_ordinal);
        let outline_color = theme.colors.bar.pos_indicator_color;
        let outline_width = theme.sizes.bar.pos_indicator_outline;
        let offset = if self.bar_layout.size.width <= 0.0 {
            BevyUtil::offscreen_offset()
        } else {
            let y = self.bar_layout.offset.y;
            Vec3::new(
                self.offset_x(),
                y + theme.sizes.bar.bar_separator_extra,
                theme.core.pos_indicator_z,
            )
        };
        OutlineRectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::TopLeft,
            color,
            outline_width,
            outline_color,
            offset,
        }
    }
}


impl PosIndicatorData {
    pub fn update_pos(
        commands: &mut Commands,
        theme: &NotationTheme,
        pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
        pos: Position,
    ) -> Option<PosIndicatorData> {
        if let Ok((entity, mut data)) = pos_indicator_query.single_mut() {
            data.bar_position = pos.bar;
            data.update(commands, theme, entity);
            Some(data.clone())
        } else {
            None
        }
    }
}
