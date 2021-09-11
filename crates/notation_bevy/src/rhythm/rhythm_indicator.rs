use std::f32::consts::PI;
use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::BevyUtil;
use notation_model::prelude::{Signature, TabBarProps, Units};

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct RhythmIndicatorValue {
    pub signature: Signature,
    pub bar_radius: f32,
    pub in_bar_pos: Units,
}
impl Display for RhythmIndicatorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RhythmIndicatorValue>{:?}", self)
    }
}
pub type RhythmIndicatorData = BarData<RhythmIndicatorValue>;

impl RhythmIndicatorData {
    pub fn angle(&self) -> f32 {
        let bar_units = Units::from(self.value.signature);
        -PI * 2.0 * (self.value.in_bar_pos.0 / bar_units.0)
    }
    pub fn shape(&self, theme: &NotationTheme) -> shapes::SvgPathShape {
        let width = self.value.bar_radius * theme.sizes.tab_control.rhythm_indicator_width_factor;
        let radius = self.value.bar_radius * theme.sizes.tab_control.rhythm_indicator_radius_factor;
        let path = format!(
            "M {} {} L {} {} L {} {} Z",
            radius,
            0.0,
            radius + width / 2.0,
            radius,
            radius - width / 2.0,
            radius
        );
        shapes::SvgPathShape {
            svg_doc_size_in_px: Vec2::new(radius * 2.0, radius * 2.0),
            svg_path_string: path,
        }
    }
}

pub struct RhythmIndicator<'a> {
    theme: &'a NotationTheme,
    data: RhythmIndicatorData,
}

impl<'a> LyonShape<shapes::SvgPathShape> for RhythmIndicator<'a> {
    fn get_name(&self) -> String {
        format!("| {}", self.data.bar_props.section_ordinal)
    }
    fn get_shape(&self) -> shapes::SvgPathShape {
        self.data.shape(self.theme)
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(
            self.theme
                .colors
                .of_section(self.data.bar_props.section_index),
        )
        /*
        ShapeColors::new(self.theme.colors.rhythm.indicator)
         */
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.sizes.tab_control.rhythm_indicator_line_width;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        if self.data.value.bar_radius <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            rotation: Quat::from_rotation_z(self.data.angle()),
            scale: Vec3::ONE,
        }
    }
}

impl<'a>
    LyonShapeOp<'a, NotationTheme, RhythmIndicatorData, shapes::SvgPathShape, RhythmIndicator<'a>>
    for RhythmIndicator<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: RhythmIndicatorData) -> RhythmIndicator<'a> {
        RhythmIndicator::<'a> { theme, data }
    }
}

impl<'a> RhythmIndicator<'a> {
    pub fn update_size(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        data: &mut RhythmIndicatorData,
        bar_radius: f32,
    ) {
        data.value.bar_radius = bar_radius;
        RhythmIndicator::update(commands, theme, entity, data);
    }
    pub fn spawn(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        bar_props: TabBarProps,
        signature: Signature,
    ) -> Entity {
        let indicator_value = RhythmIndicatorValue {
            signature,
            bar_radius: 0.0,
            in_bar_pos: Units(0.0),
        };
        let indicator_data = RhythmIndicatorData {
            bar_props,
            value: indicator_value,
        };
        let indicator_entity = RhythmIndicator::create(commands, theme, entity, indicator_data);
        indicator_entity
    }
}
