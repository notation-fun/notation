use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, LayoutSize};
use notation_model::prelude::Finger;

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct FretFingerData {
    pub string: u8,
    pub fret: Option<u8>,
    pub finger: Option<Finger>,
    pub guitar_size: LayoutSize,
}

impl FretFingerData {
    pub fn new(string: u8, fret: Option<u8>, finger: Option<Finger>) -> Self {
        FretFingerData {
            string,
            fret,
            finger,
            guitar_size: LayoutSize::ZERO,
        }
    }
    pub fn should_hide(&self, default_fret: u8) -> bool {
        self.fret.is_some() && self.fret.unwrap() == default_fret
    }
}
pub struct FretFinger<'a> {
    theme: &'a NotationTheme,
    data: FretFingerData,
}

impl<'a> LyonShape<shapes::Circle> for FretFinger<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{:?}{:?}",
            self.data.string, self.data.fret, self.data.finger,
        )
    }
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            radius: self.theme.guitar.fret_finger_radius,
            center: Vec2::ZERO,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = if self.data.fret.is_none() {
            self.theme.shapes.shape_finger_mute_color
        } else {
            self.theme.shapes.shape_finger_color
        };
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        //TODO: calc with fretboard information for muted fret
        if self.data.should_hide(0) || self.data.guitar_size.width <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let x = self.theme.guitar.calc_string_x(self.data.string, self.data.guitar_size.width);
        let fret = self.data.fret.unwrap_or(0);
        let y = self.theme.guitar.calc_fret_y(fret, self.data.guitar_size.height);
        Transform::from_xyz(x, y, self.theme.core.mini_bar_z + 2.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, FretFingerData, shapes::Circle, FretFinger<'a>>
    for FretFinger<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: FretFingerData) -> FretFinger<'a> {
        FretFinger::<'a> { theme, data }
    }
}
