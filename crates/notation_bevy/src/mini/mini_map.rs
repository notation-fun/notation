use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::fmt::Display;

use crate::prelude::{
    LyonShape, LyonShapeOp, NotationAppState, NotationLayout, NotationSettings, NotationTheme,
};
use bevy_utils::prelude::{
    DockPanel, DockSide, LayoutAnchor, LayoutChangedQuery, LayoutConstraint, LayoutSize, View,
};

use super::mini_bar::{MiniBarData, MiniBarLayout, MiniBarShape};
use super::mini_section_separator::{MiniSectionSeparator, MiniSectionSeparatorData};

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
        Self { bars }
    }
    pub fn calc_mini_bar_layout(
        &self,
        engine: &NotationLayout,
        constraint: LayoutConstraint,
    ) -> MiniBarLayout {
        let sizes = engine.theme.sizes.mini_map;
        let content_width = constraint.max.width - sizes.margin * 2.0;
        if self.bars == 0 || content_width < sizes.min_bar_width {
            return MiniBarLayout::new(0, 0, sizes.max_bar_width, 0.0);
        }
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
        let height =
            layout.rows as f32 * sizes.bar_height + (layout.rows + 1) as f32 * sizes.margin;
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

impl<'a> LyonShapeOp<'a, NotationTheme, MiniMapBackData, shapes::Rectangle, MiniMapBack<'a>>
    for MiniMapBack<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: MiniMapBackData) -> MiniMapBack<'a> {
        MiniMapBack::<'a> { theme, data }
    }
}

impl MiniMap {
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        query: LayoutChangedQuery<MiniMap>,
        mut mini_map_back_query: Query<(Entity, &mut MiniMapBackData)>,
        mut mini_bar_query: Query<(Entity, &mut MiniBarData)>,
        mut mini_section_separator_query: Query<(Entity, &mut MiniSectionSeparatorData)>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for (_entity, minimap, minimap_layout) in query.iter() {
            let bar_layout = minimap.calc_mini_bar_layout(&engine, minimap_layout.size.into());
            if let Ok((back_entity, mut back_data)) = mini_map_back_query.single_mut() {
                *back_data =
                    MiniMapBackData::new(minimap_layout.size.width, minimap_layout.size.height);
                MiniMapBack::update(&mut commands, &theme, back_entity, &back_data);
            }
            for (entity, mut data) in mini_bar_query.iter_mut() {
                data.value.layout = bar_layout.clone();
                MiniBarShape::update(&mut commands, &theme, entity, &data);
            }
            for (entity, mut data) in mini_section_separator_query.iter_mut() {
                data.value.layout = bar_layout.clone();
                MiniSectionSeparator::update(&mut commands, &theme, entity, &data);
            }
        }
    }
}
