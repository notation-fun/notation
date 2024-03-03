use edger_bevy::bevy_prelude::*;
use edger_bevy::bevy_prototype_lyon::prelude::*;
use edger_bevy::prelude::{offscreen, FillRectangle, LayoutSize, ShapeOp};
use notation_model::prelude::{HandShape6, Pick};

use crate::prelude::NotationTheme;

#[derive(Clone, Debug, Component)]
pub struct GuitarBarreData {
    pub capo: u8,
    pub shape: Option<HandShape6>,
    pub pick: Option<Pick>,
    pub view_size: LayoutSize,
    pub guitar_size: LayoutSize,
}

impl Default for GuitarBarreData {
    fn default() -> Self {
        Self {
            capo: 0,
            shape: None,
            pick: None,
            view_size: LayoutSize::ZERO,
            guitar_size: LayoutSize::ZERO,
        }
    }
}
impl GuitarBarreData {
    pub fn update_pick(&mut self, pick: &Pick) {
        if self.pick.is_none() || self.pick.unwrap().max_fret() < pick.max_fret() {
            self.pick = Some(pick.clone());
        }
    }
    pub fn barre(&self) -> u8 {
        self.shape.and_then(|x| x.barre).unwrap_or(0)
    }
    pub fn max_fret(&self) -> u8 {
        let mut max = self.shape.map(|x| x.max_fret_with_barre()).unwrap_or(0);
        if let Some(pick) = self.pick {
            let pick_max = pick.max_fret();
            if pick_max > max {
                max = pick_max
            }
        }
        max + self.capo
    }
}

impl ShapeOp<NotationTheme, FillRectangle> for GuitarBarreData {
    fn get_shape(&self, theme: &NotationTheme) -> FillRectangle {
        let width = self.guitar_size.width * theme.guitar.barre_width_factor;
        let height = self.guitar_size.height * theme.guitar.barre_height_factor;
        let color = theme.colors.strings.barre;
        let offset = if self.barre() == 0 || self.guitar_size.width <= 0.0 {
            offscreen::offset()
        } else {
            let finger_radius = theme.guitar.string_x_factor * self.guitar_size.width / 2.0;
            let y = theme
                .guitar
                .calc_fret_y(self.capo + self.barre(), self.guitar_size.height)
                + height * 0.5
                - finger_radius;
            Vec3::new(0.0, y, theme.z.guitar_barre)
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
