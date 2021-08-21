use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::fmt::Display;

use bevy_utils::prelude::{DockPanel, DockSide, LayoutAnchor, LayoutConstraint, LayoutSize, View};
use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme, NotationLayout};

use super::mini_bar::MiniBarLayout;

#[derive(Clone, Debug, Default)]
pub struct MiniMap {
    pub bars: usize,
}
impl Display for MiniMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Minimap> {}", self.bars)
    }
}
impl MiniMap {
    pub fn new(bars: usize) -> Self {
        Self {
            bars,
        }
    }
    pub fn calc_mini_bar_layout(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> MiniBarLayout {
        let sizes = engine.theme.sizes.mini_map;
        if self.bars == 0 {
            return MiniBarLayout::new(0, 0, sizes.max_bar_width, 0.0);
        }
        let content_width = constraint.max.width - sizes.margin * 2.0;
        let mut width = content_width / self.bars as f32;
        let mut rows = 1;
        let mut cols = self.bars;
        if width < sizes.min_bar_width {
            width = sizes.min_bar_width;
            cols = (content_width / width).floor() as usize;
            rows = self.bars / cols;
            if self.bars % cols > 0 {
                rows += 1;
            }
        } else if width > sizes.max_bar_width {
            width = sizes.max_bar_width;
        }
        let space = constraint.max.width - width * cols as f32;
        MiniBarLayout::new(rows, cols, width, space / 2.0)
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for MiniMap {
    fn dock_side(&self) -> DockSide {
        DockSide::Bottom
    }
}
impl<'a> View<NotationLayout<'a>> for MiniMap {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::BOTTOM_LEFT
    }
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let sizes = engine.theme.sizes.mini_map;
        let layout = self.calc_mini_bar_layout(engine, constraint);
        let height = layout.rows as f32 * sizes.bar_height
            + (layout.rows + 1) as f32 * sizes.margin;
        LayoutSize::new(constraint.max.width, height)
    }
}

#[derive(Clone, Debug, Default)]
pub struct MiniMapBackData {
    pub width: f32,
    pub height: f32,
}
impl MiniMapBackData {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
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
            origin: shapes::RectangleOrigin::Center,
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
        let x = self.data.width / 2.0;
        let y = self.data.height / 2.0;
        Transform::from_xyz(x, y, self.theme.core.mini_map_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, MiniMapBackData, shapes::Rectangle, MiniMapBack<'a>> for MiniMapBack<'a> {
    fn new_shape(theme: &'a NotationTheme, data: MiniMapBackData) -> MiniMapBack<'a> {
        MiniMapBack::<'a> { theme, data }
    }
}
