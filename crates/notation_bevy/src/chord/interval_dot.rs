use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{IntervalQuality};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct IntervalDotData {
    pub quality: IntervalQuality,
    pub total: usize,
    pub index: usize,
    pub note_radius: f32,
}

impl IntervalDotData {
    pub fn new(
        quality: IntervalQuality,
        total: usize,
        index: usize,
        note_radius: f32,
    ) -> Self {
        Self {
            quality,
            total,
            index,
            note_radius,
        }
    }
    fn circle_offset(total: usize, index: usize, note_radius: f32, factor: f32) -> Vec2 {
        let angle_offset = match total {
            2 => -90,
            3 => -150,
            4 => -135,
            _ => 0,
        } as f32
            * PI
            / 180.0;
        let angle = PI * 2.0 * index as f32 / total as f32 + angle_offset;
        Vec2::new(
            note_radius * factor * angle.cos(),
            note_radius * factor * angle.sin(),
        )
    }
    fn offset(&self, theme: &NotationTheme) -> Vec2 {
        if self.total == 0 || self.note_radius <= 0.0 {
            return Vec2::ZERO;
        }
        if self.index == 0 {
            if self.total == 1 || self.total == 5 || self.total == 7 {
                return Vec2::ZERO;
            }
        }
        if self.total != 6 {
            let (minus1, factor) = match self.total {
                2 => (false, theme.sizes.chord.interval_dot_offset_2_factor),
                3 | 4 => (false, theme.sizes.chord.interval_dot_offset_3_4_factor),
                5 | 7 => (true, theme.sizes.chord.interval_dot_offset_5_7_factor),
                _ => (false, 0.0),
            };
            if minus1 {
                Self::circle_offset(self.total - 1, self.index - 1, self.note_radius, factor)
            } else {
                Self::circle_offset(self.total, self.index, self.note_radius, factor)
            }
        } else {
            let x = ((self.index % 2) as f32 - 0.5) * 2.0
                * self.note_radius * theme.sizes.chord.interval_dot_offset_6_factor.0;
            let y = ((self.index % 3) as f32 - 1.0)
                * self.note_radius * theme.sizes.chord.interval_dot_offset_6_factor.1;
            Vec2::new(x, y)
        }
    }
}
pub struct IntervalDot<'a> {
    theme: &'a NotationTheme,
    data: IntervalDotData,
}

impl<'a> LyonShape<shapes::Circle> for IntervalDot<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{}/{}",
            self.data.quality, self.data.index, self.data.total,
        )
    }
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            radius: self.data.note_radius * self.theme.sizes.chord.interval_dot_radius_factor,
            center: Vec2::ZERO,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.colors.chord.dot.of_quality(&self.data.quality))
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        let offset = self.data.offset(self.theme);
        Transform::from_xyz(offset.x, offset.y, 1.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, IntervalDotData, shapes::Circle, IntervalDot<'a>>
    for IntervalDot<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: IntervalDotData) -> IntervalDot<'a> {
        IntervalDot::<'a> { theme, data }
    }
}
