use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::Tab;
use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{
    LyonShape, LyonShapeOp, NotationAppState, NotationLayout, NotationSettings, NotationTheme,
};
use bevy_utils::prelude::{
    BevyUtil, DockPanel, DockSide, GridCellSize, GridData, GridView,
    LayoutChangedQuery, LayoutConstraint, LayoutData, LayoutQuery, LayoutSize, View,
    ViewAddedQuery, ViewBundle, ViewQuery,
};

use super::mini_bar::{MiniBar};
use super::mini_plugin::MiniMapDoLayoutEvent;

#[derive(Clone, Debug)]
pub struct MiniMap {
    pub tab: Arc<Tab>,
}
impl Display for MiniMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Minimap> {}", self.tab)
    }
}
impl MiniMap {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
    pub fn calc_grid_data(
        &self,
        engine: &NotationLayout,
        constraint: LayoutConstraint,
    ) -> GridData {
        let sizes = engine.theme.sizes.mini_map;
        let content_width = constraint.max.width - sizes.margin * 2.0;
        let bars = self.tab.bars.len();
        if bars == 0 || content_width < sizes.min_bar_width {
            return GridData::ZERO;
        }
        let mut width = content_width / bars as f32;
        let mut rows = 1;
        let mut cols = bars;
        if width < sizes.min_bar_width {
            width = sizes.min_bar_width;
            cols = (content_width / width).floor() as usize;
            rows = bars / cols;
            if bars % cols > 0 {
                rows += 1;
            }
        } else if width > sizes.max_bar_width {
            width = sizes.max_bar_width;
        }
        let space = constraint.max.width - width * cols as f32;
        GridData {
            rows,
            cols,
            size: GridCellSize::Fixed(LayoutSize::new(width, sizes.bar_height)),
            offset: Vec2::new(space / 2.0, -engine.theme.sizes.mini_map.margin / 2.0),
        }
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for MiniMap {
    fn dock_side(&self) -> DockSide {
        DockSide::Bottom
    }
}
impl<'a> View<NotationLayout<'a>> for MiniMap {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let sizes = engine.theme.sizes.mini_map;
        let grid_data = self.calc_grid_data(engine, constraint);
        let height = grid_data.rows as f32 * sizes.bar_height + sizes.margin;
        LayoutSize::new(constraint.max.width, height)
    }
}
impl<'a> GridView<NotationLayout<'a>, MiniBar> for MiniMap {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, data: LayoutData) -> GridData {
        self.calc_grid_data(&engine, data.into())
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
        Transform::from_xyz(0.0, 0.0, self.theme.core.mini_map_z)
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
    pub fn spawn(
        commands: &mut Commands,
        theme: &NotationTheme,
        tab_entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let minimap = MiniMap::new(tab.clone());
        let back_data = MiniMapBackData::default();
        let map_entity =
            BevyUtil::spawn_child_bundle(commands, tab_entity, ViewBundle::from(minimap));
        MiniMapBack::create(commands, theme, map_entity, back_data);
        map_entity
    }
    pub fn on_added(
        mut commands: Commands,
        _theme: Res<NotationTheme>,
        query: ViewAddedQuery<MiniMap>,
    ) {
        for (_parent, entity, view) in query.iter() {
            for bar in view.tab.bars.iter() {
                BevyUtil::spawn_child_bundle(
                    &mut commands,
                    entity,
                    ViewBundle::from(MiniBar::new(bar, bar.clone())),
                );
            }
        }
    }
    pub fn do_layout(
        mut evts: EventReader<MiniMapDoLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<MiniBar>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &engine,
                &mut layout_query,
                &cell_query,
                evt.entity,
                evt.layout,
            )
        }
    }
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedQuery<MiniMap>,
        mut mini_map_back_query: Query<(Entity, &mut MiniMapBackData)>,
        mut evts: EventWriter<MiniMapDoLayoutEvent>,
    ) {
        for (entity, view, layout) in query.iter() {
            if let Ok((back_entity, mut back_data)) = mini_map_back_query.single_mut() {
                *back_data = MiniMapBackData::new(layout.size.width, layout.size.height);
                MiniMapBack::update(&mut commands, &theme, back_entity, &back_data);
            }
            evts.send(MiniMapDoLayoutEvent::new(entity, view, layout))
        }
    }
}
