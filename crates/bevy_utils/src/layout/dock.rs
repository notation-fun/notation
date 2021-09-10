use bevy::prelude::*;

use crate::prelude::{
    LayoutAnchor, LayoutConstraint, LayoutData, LayoutEnv, LayoutQuery, LayoutSize, View, ViewQuery,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockSide {
    Top,
    Bottom,
    Left,
    Right,
}
impl From<DockSide> for LayoutAnchor {
    fn from(v: DockSide) -> Self {
        match v {
            DockSide::Top => LayoutAnchor::TOP,
            DockSide::Bottom => LayoutAnchor::BOTTOM,
            DockSide::Left => LayoutAnchor::LEFT,
            DockSide::Right => LayoutAnchor::RIGHT,
        }
    }
}
impl DockSide {
    pub fn is_h(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
    pub fn is_v(&self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }
}

pub trait DockPanel<TE: LayoutEnv>: View<TE> {
    fn dock_side(&self, engine: &TE, size: LayoutSize) -> DockSide;
}

pub trait DockView<TE: LayoutEnv, TP: DockPanel<TE>, TC: View<TE>>: View<TE> {
    fn do_layout(
        &self,
        engine: &TE,
        layout_query: &mut LayoutQuery,
        panel_query: &ViewQuery<TP>,
        content_query: &ViewQuery<TC>,
        entity: Entity,
        data: LayoutData,
    ) {
        let panel = engine.get_child(panel_query, entity);
        let content = engine.get_child(content_query, entity);
        if panel.is_none() || content.is_none() {
            return;
        }
        let panel = panel.unwrap();
        let content = content.unwrap();
        let panel_constraint = LayoutConstraint::from(data);
        let panel_size = panel.view.calc_size(engine, panel_constraint);
        let dock_side = panel.view.dock_side(engine, data.size);
        let panel_anchor = LayoutAnchor::from(dock_side);
        panel.set_layout_data(
            layout_query,
            data.new_child(panel_anchor, Vec2::ZERO, panel_size),
        );
        let content_anchor = panel_anchor.opposite();
        let content_size = if dock_side.is_h() {
            data.size - LayoutSize::new(panel_size.width, 0.0)
        } else {
            data.size - LayoutSize::new(0.0, panel_size.height)
        };
        content.set_layout_data(
            layout_query,
            data.new_child(content_anchor, Vec2::ZERO, content_size),
        );
    }
}
