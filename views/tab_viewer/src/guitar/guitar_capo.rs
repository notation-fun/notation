use edger_bevy::bevy_prelude::*;
use edger_bevy::bevy_prototype_lyon::prelude::*;
use edger_bevy::prelude::{offscreen, FillRectangle, LayoutSize, ShapeOp};

use crate::prelude::NotationTheme;

#[derive(Clone, Debug, Component)]
pub struct GuitarCapoData {
    pub capo: u8,
    pub view_size: LayoutSize,
    pub guitar_size: LayoutSize,
}

impl Default for GuitarCapoData {
    fn default() -> Self {
        Self {
            capo: 0,
            view_size: LayoutSize::ZERO,
            guitar_size: LayoutSize::ZERO,
        }
    }
}

impl ShapeOp<NotationTheme, FillRectangle> for GuitarCapoData {
    fn get_shape(&self, theme: &NotationTheme) -> FillRectangle {
        let width = self.guitar_size.width * theme.guitar.capo_width_factor;
        let height = self.guitar_size.height * theme.guitar.capo_height_factor;
        let color = theme.colors.strings.capo;
        let offset = if self.capo == 0 || self.guitar_size.width <= 0.0 {
            offscreen::offset()
        } else {
            let finger_radius = theme.guitar.string_x_factor * self.guitar_size.width / 2.0;
            let y = theme.guitar.calc_fret_y(self.capo, self.guitar_size.height) + height * 0.5
                - finger_radius;
            Vec3::new(0.0, y, theme.z.guitar_capo)
        };
        FillRectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::Center,
            color,
            offset,
        }
    }
}
