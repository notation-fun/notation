use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::Tab;
use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{
    LyonShape, LyonShapeOp, NotationAppState, NotationLayout, NotationSettings, NotationTheme,
};
use bevy_utils::prelude::{BevyUtil, DockPanel, DockSide, GridData, GridView, LayoutAnchor, LayoutChangedQuery, LayoutConstraint, LayoutData, LayoutQuery, LayoutSize, View, ViewAddedQuery, ViewBundle, ViewQuery};

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
        grid_size: LayoutSize,
    ) -> GridData {
        let sizes = engine.theme.sizes.mini_map;
        let (rows, cols, cell_width) = GridData::cals_fixed_rows_cols_by_width(
            grid_size.width, sizes.bar_width_range, sizes.bar_margin().width, self.tab.bars.len());
        let size = LayoutSize::new(cell_width, sizes.bar_height);
        GridData::new_fixed(grid_size, rows, cols, size, sizes.bar_margin(), LayoutAnchor::TOP_LEFT)
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for MiniMap {
    fn dock_side(&self) -> DockSide {
        DockSide::Bottom
    }
}
impl<'a> View<NotationLayout<'a>> for MiniMap {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let grid_data = self.calc_grid_data(engine, constraint.max);
        let height = grid_data.content_size().height + engine.theme.sizes.mini_map.bar_margin().height * 2.0;
        LayoutSize::new(constraint.max.width, height)
    }
}
impl<'a> GridView<NotationLayout<'a>, MiniBar> for MiniMap {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        self.calc_grid_data(&engine, grid_size)
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
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<MiniBar>,
        mut mini_map_back_query: Query<(Entity, &mut MiniMapBackData)>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            if let Ok((back_entity, mut back_data)) = mini_map_back_query.single_mut() {
                *back_data = MiniMapBackData::new(evt.layout.size.width, evt.layout.size.height);
                MiniMapBack::update(&mut commands, &theme, back_entity, &back_data);
            }
            evt.view.do_layout(
                &mut commands,
                &engine,
                &mut layout_query,
                &cell_query,
                evt.entity,
                evt.layout,
            )
        }
    }
}
