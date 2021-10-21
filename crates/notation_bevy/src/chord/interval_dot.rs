use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{OutlineCircle, ShapeOp};
use notation_model::prelude::IntervalQuality;

use crate::prelude::{NotationTheme};

#[derive(Clone, Debug)]
pub struct IntervalDotData {
    pub quality: IntervalQuality,
    pub total: usize,
    pub index: usize,
    pub note_radius: f32,
}

impl IntervalDotData {
    pub fn new(quality: IntervalQuality, total: usize, index: usize, note_radius: f32) -> Self {
        Self {
            quality,
            total,
            index,
            note_radius,
        }
    }
    fn circle_offset(total: usize, index: usize, note_radius: f32, factor: f32) -> Vec3 {
        let angle_offset = match total {
            2 => -90,
            3 => -150,
            4 => -135,
            _ => 0,
        } as f32
            * PI
            / 180.0;
        let angle = PI * 2.0 * index as f32 / total as f32 + angle_offset;
        Vec3::new(
            note_radius * factor * angle.cos(),
            note_radius * factor * angle.sin(),
            1.0,
        )
    }
    fn offset(&self, theme: &NotationTheme) -> Vec3 {
        if self.total == 0 || self.note_radius <= 0.0 {
            return Vec3::ZERO;
        }
        if self.index == 0 {
            if self.total == 1 || self.total == 5 || self.total == 7 {
                return Vec3::ZERO;
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
            let x = ((self.index % 2) as f32 - 0.5)
                * 2.0
                * self.note_radius
                * theme.sizes.chord.interval_dot_offset_6_factor.0;
            let y = ((self.index % 3) as f32 - 1.0)
                * self.note_radius
                * theme.sizes.chord.interval_dot_offset_6_factor.1;
            Vec3::new(x, y, 1.0)
        }
    }
}

impl ShapeOp<NotationTheme, shapes::Circle, OutlineCircle> for IntervalDotData {
    fn get_shape(&self, theme: &NotationTheme) -> OutlineCircle {
        let radius = if self.total == 1 {
            self.note_radius * theme.sizes.chord.interval_dot_big_radius_factor
        } else {
            self.note_radius * theme.sizes.chord.interval_dot_radius_factor
        };
        let color = theme.colors.chord.dot.of_quality(&self.quality);
        let outline_width = theme.sizes.chord.interval_dot_outline;
        let outline_color = theme
                    .colors
                    .chord
                    .dot_outline
                    .of_quality(&self.quality);
        OutlineCircle {
            radius,
            color,
            outline_width,
            outline_color,
            offset: self.offset(theme),
        }
    }
}
