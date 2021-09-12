use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, LayoutData};
use notation_model::prelude::{BarPosition, Position, TabBarProps, Units};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

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

pub struct PosIndicator<'a> {
    pub theme: &'a NotationTheme,
    pub data: PosIndicatorData,
}

impl<'a> LyonShape<shapes::Rectangle> for PosIndicator<'a> {
    fn get_name(&self) -> String {
        "Current Pos".to_string()
    }
    fn get_shape(&self) -> shapes::Rectangle {
        let width = self.theme.sizes.bar.pos_indicator_size;
        let height = self.data.bar_layout.size.height + self.theme.sizes.bar.bar_separator_extra * 2.0;
        shapes::Rectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::TopLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::outlined(
            self.theme
                    .colors
                    .of_section(self.data.bar_props.section_index),
            self.theme.colors.bar.pos_indicator_color,
        )
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(self.theme.sizes.bar.pos_indicator_outline),
        }
    }
    fn get_transform(&self) -> Transform {
        if self.data.bar_layout.size.width <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let y = self.data.bar_layout.offset.y;
        Transform::from_xyz(
            self.data.offset_x(),
            y + self.theme.sizes.bar.bar_separator_extra,
            self.theme.core.pos_indicator_z,
        )
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, PosIndicatorData, shapes::Rectangle, PosIndicator<'a>>
    for PosIndicator<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: PosIndicatorData) -> PosIndicator<'a> {
        PosIndicator::<'a> { theme, data }
    }
}

impl<'a> PosIndicator<'a> {
    pub fn update_pos(
        commands: &mut Commands,
        theme: &'a NotationTheme,
        pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
        pos: Position,
    ) -> Option<PosIndicatorData> {
        if let Ok((entity, mut data)) = pos_indicator_query.single_mut() {
            data.bar_position = pos.bar;
            Self::update(commands, theme, entity, &data);
            Some(data.clone())
        } else {
            None
        }
    }
}
