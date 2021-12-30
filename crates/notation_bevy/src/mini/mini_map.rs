use bevy::prelude::*;
use notation_model::prelude::Tab;
use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{
    NotationAppState, NotationAssets, NotationLayout, NotationSettings, NotationTheme,
};
use notation_bevy_utils::prelude::{
    BevyUtil, ColorBackground, DockPanel, DockSide, GridData, GridView, LayoutAnchor,
    LayoutConstraint, LayoutQuery, LayoutSize, View, ViewBundle, ViewQuery,
};

use super::mini_bar::MiniBar;
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
    pub fn calc_grid_data(&self, engine: &NotationLayout, grid_size: LayoutSize) -> GridData {
        let sizes = engine.theme.sizes.mini_map;
        let (rows, cols, cell_width) = GridData::cals_fixed_rows_cols_by_width(
            grid_size.width - sizes.bar_margin.1 * 2.0,
            sizes.bar_width_range,
            sizes.bar_margin().width,
            self.tab.bars.len(),
        );
        let size = LayoutSize::new(cell_width, sizes.bar_height);
        GridData::new_fixed(
            rows,
            cols,
            size,
            sizes.bar_margin(),
            LayoutAnchor::TOP_LEFT,
            grid_size,
        )
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for MiniMap {
    fn dock_side(&self, _engine: &NotationLayout<'a>, _size: LayoutSize) -> DockSide {
        DockSide::Bottom
    }
}
impl<'a> View<NotationLayout<'a>> for MiniMap {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let grid_data = self.calc_grid_data(engine, constraint.max);
        let height =
            grid_data.content_size().height + engine.theme.sizes.mini_map.bar_margin().height * 2.0;
        LayoutSize::new(constraint.max.width, height)
    }
}
impl<'a> GridView<NotationLayout<'a>, MiniBar> for MiniMap {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        self.calc_grid_data(&engine, grid_size)
    }
}

impl MiniMap {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let minimap = MiniMap::new(tab.clone());
        let map_entity = BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(minimap));
        let background_entity = ColorBackground::spawn(
            commands,
            map_entity,
            theme.z.mini_map,
            theme.colors.mini_map.back,
        );
        for bar in tab.bars.iter() {
            MiniBar::spawn(commands, assets, theme, map_entity, bar);
        }
        theme
            .texts
            .mini_map
            .spawn_debug_text(commands, background_entity, &assets, "");
        map_entity
    }
    pub fn do_layout(
        mut evts: EventReader<MiniMapDoLayoutEvent>,
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<MiniBar>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
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
    pub fn update_debug_str(
        app_state: Res<NotationAppState>,
        background_query: Query<&ColorBackground>,
        mut font_query: Query<(&Parent, &mut Text)>,
    ) {
        for (parent, mut text) in font_query.iter_mut() {
            if let Ok(_) = background_query.get(parent.0) {
                let str = if let Some(debug_str) = &app_state.debug_str {
                    debug_str.to_string()
                } else {
                    "".to_string()
                };
                BevyUtil::set_text_value(&mut text, str);
            }
        }
    }
}
