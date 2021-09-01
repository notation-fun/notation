use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, LayoutSize};
use notation_model::prelude::PlayingState;

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct GuitarStringData {
    pub string: u8,
    pub guitar_size: LayoutSize,
    pub state: PlayingState,
    pub hit: bool,
    pub hit_second: f64,
}

impl GuitarStringData {
    pub fn new(string: u8) -> Self {
        Self {
            string: string,
            guitar_size: LayoutSize::ZERO,
            state: PlayingState::Idle,
            hit: false,
            hit_second: 0.0,
        }
    }
    pub fn set_hit(&mut self, hit: bool, time: &Time, hit_string_seconds: f64) {
        if self.hit && !hit {
            if (time.seconds_since_startup() - self.hit_second) > hit_string_seconds {
                self.hit = false;
                self.hit_second = time.seconds_since_startup();
            }
        } else {
            self.hit = hit;
        }
    }
}

pub struct GuitarString<'a> {
    theme: &'a NotationTheme,
    data: GuitarStringData,
}

impl<'a> LyonShape<shapes::Line> for GuitarString<'a> {
    fn get_name(&self) -> String {
        format!(
            "<GuitarString>({})",
            self.data.string
        )
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(
            Vec2::new(0.0, self.data.guitar_size.height * self.theme.guitar.string_y_factor),
            Vec2::new(0.0, -self.data.guitar_size.height / 2.0)
        )
    }
    fn get_colors(&self) -> ShapeColors {
        let color = if self.data.hit {
            self.theme.colors.strings.hit
        } else {
            self.theme.colors.strings.string.of_state(&self.data.state)
        };
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.guitar.get_string_width(self.data.string);
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        if self.data.guitar_size.width <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let x = -1.0 * (self.data.string as f32 - 3.5) * self.data.guitar_size.width * self.theme.guitar.string_x_factor;
        Transform::from_xyz(x, 0.0, self.theme.core.mini_bar_z + 1.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, GuitarStringData, shapes::Line, GuitarString<'a>>
    for GuitarString<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: GuitarStringData) -> GuitarString<'a> {
        GuitarString::<'a> { theme, data }
    }
}
