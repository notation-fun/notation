use std::f32::consts::PI;
use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy_utils::prelude::{FillCircle, ShapeOp};
use notation_model::prelude::{Signature, TabBarProps, Units};

use crate::prelude::{BarData, NotationTheme};

#[derive(Clone, Debug)]
pub struct RhythmBeatValue {
    pub signature: Signature,
    pub index: u8,
    pub bar_radius: f32,
    pub in_bar_pos: Units,
}
impl Display for RhythmBeatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RhythmBeatValue>({})", self.signature)
    }
}

pub type RhythmBeatData = BarData<RhythmBeatValue>;

impl RhythmBeatData {
    pub fn offset(&self, theme: &NotationTheme) -> Vec3 {
        let total = self.value.signature.bar_beats;
        let angle_offset = PI / 2.0;
        let angle = -PI * 2.0 * self.value.index as f32 / total as f32 + angle_offset;
        let factor = theme.sizes.tab_control.rhythm_beat_offset_factor;
        Vec3::new(
            self.value.bar_radius * factor * angle.cos(),
            self.value.bar_radius * factor * angle.sin(),
            2.0,
        )
    }
    pub fn scale(&self, theme: &NotationTheme) -> f32 {
        let bar_units = Units::from(self.value.signature);
        let beat_units = Units::from(self.value.signature.beat_unit);
        let center = Units(self.value.index as f32 * beat_units.0);
        let mut in_bar_pos = self.value.in_bar_pos;
        if self.value.index == 0 {
            if bar_units.0 - in_bar_pos.0 < beat_units.0 / 2.0 {
                in_bar_pos = in_bar_pos - bar_units;
            }
        }
        let delta = (in_bar_pos - center).0.abs();
        if delta < beat_units.0 / 2.0 {
            // https://math.stackexchange.com/questions/121720/ease-in-out-function/121755#121755
            let x = 1.0 - delta / beat_units.0 * 2.0;
            let y = x * x * (3.0 - 2.0 * x);
            1.0 + y * (theme.sizes.tab_control.rhythm_beat_max_scale - 1.0)
        } else {
            1.0
        }
    }
}

impl ShapeOp<NotationTheme, shapes::Circle, FillCircle> for RhythmBeatData {
    fn get_shape(&self, theme: &NotationTheme) -> FillCircle {
        let radius = self.value.bar_radius
            * theme.sizes.tab_control.rhythm_beat_radius_factor
            * self.scale(theme);
        let color = theme
            .colors
            .rhythm
            .get_beat_color(&self.value.signature, self.value.index);
        let offset = self.offset(theme);
        FillCircle {
            radius,
            color,
            offset,
        }
    }
}

impl RhythmBeatData {
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
        index: u8,
    ) -> Entity {
        let beat_value = RhythmBeatValue {
            signature,
            index,
            bar_radius: 0.0,
            in_bar_pos: Units(0.0),
        };
        let beat_data = RhythmBeatData {
            bar_props,
            value: beat_value,
        };
        let beat_entity = beat_data.create(commands, theme, entity);
        beat_entity
    }
}
