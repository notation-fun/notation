use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::{Interval, Syllable};

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct ChordIntervalValue {
    pub total: usize,
    pub index: usize,
    pub size: f32,
    pub root: Syllable,
    pub interval: Interval,
}
impl ChordIntervalValue {
    pub fn calc_xy(&self) -> (f32, f32) {
        let angle_offset = match self.total {
            2 => -180,
            3 => -150,
            4 => -135,
            _ => 0,
        } as f32
            * PI
            / 180.0;
        let angle = PI * 2.0 * self.index as f32 / self.total as f32 + angle_offset;
        (self.size * 1.4 * angle.cos(), self.size * 1.4 * angle.sin())
    }
    pub fn calc_syllable(&self) -> Syllable {
        Syllable::from((self.root, self.interval))
    }
}

pub type ChordIntervalData = BarData<ChordIntervalValue>;

pub struct ChordInterval<'a> {
    theme: &'a NotationTheme,
    data: ChordIntervalData,
}

impl<'a> LyonShape<shapes::Circle> for ChordInterval<'a> {
    fn get_name(&self) -> String {
        format!("{}: {:?}", self.data.bar_props.bar_ordinal, self.data.value)
    }
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: self.data.value.size,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = self
            .theme
            .colors
            .color_of_syllable(self.data.value.calc_syllable());
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }

    fn get_transform(&self) -> Transform {
        let (x, y) = self.data.value.calc_xy();
        Transform::from_xyz(x, y, 1.0)
    }
}

impl<'a> LyonShapeOp<'a, ChordIntervalData, shapes::Circle, ChordInterval<'a>>
    for ChordInterval<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: ChordIntervalData) -> ChordInterval<'a> {
        ChordInterval::<'a> { theme, data }
    }
}
