use std::{f32::consts::PI, fmt::Display};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy_utils::prelude::BevyUtil;
use notation_model::prelude::{Signature, TabBarProps, Units};

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

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
    pub fn offset(&self, theme: &NotationTheme) -> Vec2 {
        let total = self.value.signature.bar_beats;
        let angle_offset = PI / 2.0;
        let angle = -PI * 2.0 * self.value.index as f32 / total as f32 + angle_offset;
        let factor = theme.sizes.tab_control.rhythm_beat_offset_factor;
        Vec2::new(
            self.value.bar_radius * factor * angle.cos(),
            self.value.bar_radius * factor * angle.sin(),
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

pub struct RhythmBeat<'a> {
    theme: &'a NotationTheme,
    data: RhythmBeatData,
}

impl<'a> LyonShape<shapes::Circle> for RhythmBeat<'a> {
    fn get_name(&self) -> String {
        format!("{}", self.data)
    }
    fn get_shape(&self) -> shapes::Circle {
        let radius = self.data.value.bar_radius * self.theme.sizes.tab_control.rhythm_beat_radius_factor * self.data.scale(self.theme);
        shapes::Circle {
            center: Vec2::ZERO,
            radius,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = self
            .theme
            .colors.rhythm
            .get_beat_color(&self.data.value.signature, self.data.value.index);
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        if self.data.value.bar_radius <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let offset = self.data.offset(self.theme);
        Transform::from_xyz(offset.x, offset.y, 2.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, RhythmBeatData, shapes::Circle, RhythmBeat<'a>>
    for RhythmBeat<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: RhythmBeatData) -> RhythmBeat<'a> {
        RhythmBeat::<'a> { theme, data }
    }
}

impl<'a> RhythmBeat<'a> {
    pub fn update_size(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        data: &mut RhythmBeatData,
        bar_radius: f32,
    ) {
        data.value.bar_radius = bar_radius;
        RhythmBeat::update(commands, theme, entity, data);
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
        let beat_entity = RhythmBeat::create(commands, theme, entity, beat_data);
        beat_entity
    }
}

