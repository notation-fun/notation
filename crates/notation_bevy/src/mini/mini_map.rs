use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::fmt::Display;

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct MiniMap {
    pub bars: usize,
}
impl Display for MiniMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Minimap> {}", self.bars)
    }
}

#[derive(Clone, Debug)]
pub struct MiniMapBackData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct MiniMapBack<'a> {
    theme: &'a NotationTheme,
    data: MiniMapBackData,
}

impl<'a> LyonShape<shapes::Rectangle> for MiniMapBack<'a> {
    fn get_name(&self) -> String {
        format!("{:?}", self.data)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.data.width,
            height: self.data.height,
            origin: shapes::RectangleOrigin::TopLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = self.theme.colors.mini_map.back;
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        Transform::from_xyz(self.data.x, self.data.y, self.theme.core.mini_map_z)
    }
}

impl<'a> LyonShapeOp<'a, MiniMapBackData, shapes::Rectangle, MiniMapBack<'a>> for MiniMapBack<'a> {
    fn new_shape(theme: &'a NotationTheme, data: MiniMapBackData) -> MiniMapBack<'a> {
        MiniMapBack::<'a> { theme, data }
    }
}
