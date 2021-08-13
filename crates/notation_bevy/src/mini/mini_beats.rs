use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::Syllable;

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct MiniBeatsValue {
    pub size: f32,
    pub offset: f32,
    pub syllable: Syllable,
}

pub type MiniBeatsData = BarData<MiniBeatsValue>;

pub struct MiniBeats<'a> {
    theme: &'a NotationTheme,
    data: MiniBeatsData,
}

impl<'a> LyonShape<shapes::Circle> for MiniBeats<'a> {
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
            .color_of_syllable(self.data.value.syllable);
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        Transform::from_xyz(self.data.value.offset, self.data.value.offset, 1.0)
    }
}

impl<'a> LyonShapeOp<'a, MiniBeatsData, shapes::Circle, MiniBeats<'a>> for MiniBeats<'a> {
    fn new_shape(theme: &'a NotationTheme, data: MiniBeatsData) -> MiniBeats<'a> {
        MiniBeats::<'a> { theme, data }
    }
}
