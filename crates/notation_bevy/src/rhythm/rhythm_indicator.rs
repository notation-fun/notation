use std::f32::consts::PI;
use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_bevy_utils::prelude::{ShapeOp, StrokePath};
use notation_model::prelude::{Signature, TabBarProps, Units};

use crate::prelude::{BarData, NotationTheme};

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

impl ShapeOp<NotationTheme, StrokePath> for RhythmIndicatorData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokePath {
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
        let color = theme
            .colors
            .of_section(self.bar_props.section_index);
        let line_width = theme.sizes.tab_control.rhythm_indicator_line_width;
        StrokePath {
            size: Vec2::new(radius * 2.0, radius * 2.0),
            path,
            color,
            line_width,
            offset: Vec3::new(0.0, 0.0, 1.0),
            scale: 1.0,
            angle: self.angle(),
        }
    }
}

impl RhythmIndicatorData {
    pub fn update_size(
        &mut self,
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        bar_radius: f32,
    ) {
        self.value.bar_radius = bar_radius;
        self.update(commands, theme, entity);
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
        let indicator_entity = indicator_data.create(commands, theme, entity);
        indicator_entity
    }
}
