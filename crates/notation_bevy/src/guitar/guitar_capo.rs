use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, LayoutSize};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct GuitarCapoData {
    pub capo: u8,
    pub guitar_size: LayoutSize,
}

impl GuitarCapoData {
    pub fn new(capo: u8) -> Self {
        Self {
            capo,
            guitar_size: LayoutSize::ZERO,
        }
    }
}

pub struct GuitarCapo<'a> {
    theme: &'a NotationTheme,
    data: GuitarCapoData,
}

impl<'a> LyonShape<shapes::Rectangle> for GuitarCapo<'a> {
    fn get_name(&self) -> String {
        format!("<GuitarCapo>({:?})", self.data.capo)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        let width = self.data.guitar_size.width * self.theme.guitar.capo_width_factor;
        let height = self.data.guitar_size.height * self.theme.guitar.capo_height_factor;
        shapes::Rectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::Center,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = self.theme.colors.strings.capo;
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        if self.data.capo == 0 || self.data.guitar_size.width <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let height = self.data.guitar_size.height * self.theme.guitar.capo_height_factor;
        let finger_radius = self.theme.guitar.string_x_factor * self.data.guitar_size.width / 2.0;
        let y = self
            .theme
            .guitar
            .calc_fret_y(self.data.capo, self.data.guitar_size.height)
            + height * 0.5
            - finger_radius;
        Transform::from_xyz(0.0, y, self.theme.core.mini_bar_z + 2.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, GuitarCapoData, shapes::Rectangle, GuitarCapo<'a>>
    for GuitarCapo<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: GuitarCapoData) -> GuitarCapo<'a> {
        GuitarCapo::<'a> { theme, data }
    }
}
