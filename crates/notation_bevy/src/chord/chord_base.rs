use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::Syllable;

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

use super::chord_interval::ChordIntervalValue;

#[derive(Clone, Debug)]
pub struct ChordBaseValue {
    pub interval: ChordIntervalValue,
}
impl Display for ChordBaseValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl ChordBaseValue {
    pub fn calc_syllable(&self) -> Syllable {
        self.interval.calc_syllable()
    }
}

pub type ChordBaseData = BarData<ChordBaseValue>;

pub struct ChordBase<'a> {
    theme: &'a NotationTheme,
    data: ChordBaseData,
}

impl<'a> LyonShape<shapes::Circle> for ChordBase<'a> {
    fn get_name(&self) -> String {
        format!("{}: {:?}", self.data.bar_props.bar_ordinal, self.data.value)
    }
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: self.data.value.interval.radius,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = self
            .theme
            .colors
            .of_syllable(self.data.value.calc_syllable());
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }

    fn get_transform(&self) -> Transform {
        let y = -self.data.value.interval.radius * self.theme.sizes.chords.diagram_base_y_factor;
        Transform::from_xyz(0.0, y, 1.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, ChordBaseData, shapes::Circle, ChordBase<'a>>
    for ChordBase<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: ChordBaseData) -> ChordBase<'a> {
        ChordBase::<'a> { theme, data }
    }
}
