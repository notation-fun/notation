use bevy::prelude::*;

use crate::prelude::{
    LayoutAnchor, LayoutConstraint, LayoutData, LayoutEnv, LayoutQuery, LayoutSize, View, ViewQuery,
};

pub trait VBoxCell<TE: LayoutEnv>: View<TE> {
    fn order(&self) -> usize {
        0
    }
}

#[derive(Clone, Debug, Component)]
pub struct VBoxCellData {
    pub index: usize,
}
pub trait VBoxView<TE: LayoutEnv, TC: VBoxCell<TE>>: View<TE> {
    fn sort_cells(&self) -> bool {
        false
    }
    fn do_layout(
        &self,
        commands: &mut Commands,
        engine: &TE,
        layout_query: &mut LayoutQuery,
        cell_query: &ViewQuery<TC>,
        entity: Entity,
        data: LayoutData,
    ) {
        let mut cells = engine.get_children(cell_query, entity);
        if self.sort_cells() {
            cells.sort_by(|a, b| a.view.order().cmp(&b.view.order()));
        }
        let mut y = 0.0;
        let mut height = data.size.height;
        for (index, cell) in cells.iter().enumerate() {
            let cell_constraint = LayoutConstraint::new(LayoutSize::new(data.size.width, height));
            let cell_size = cell.view.calc_size(engine, cell_constraint);
            cell.set_layout_data(
                layout_query,
                data.new_child(LayoutAnchor::TOP_LEFT, Vec2::new(0.0, y), cell_size),
            );
            commands.entity(cell.entity).insert(VBoxCellData { index });
            y -= cell_size.height;
            height -= cell_size.height;
        }
    }
}
