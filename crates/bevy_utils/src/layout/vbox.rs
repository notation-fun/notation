use bevy::prelude::*;

use crate::prelude::{LayoutAnchor, LayoutConstraint, LayoutData, LayoutEnv, LayoutQuery, LayoutSize, View, ViewQuery};

pub trait VBoxCell<TE: LayoutEnv>: View<TE> {}

#[derive(Clone, Debug)]
pub struct VBoxCellData {
    pub index: usize,
}
pub trait VBoxView<TE: LayoutEnv, TC: VBoxCell<TE>>: View<TE> {
    fn do_layout(
        &self,
        commands: &mut Commands,
        engine: &TE,
        layout_query: &mut LayoutQuery,
        cell_query: &ViewQuery<TC>,
        entity: Entity,
        data: LayoutData,
    ) {
        if self.is_root() {
            self.set_layout_data(layout_query, entity, data);
        }
        let cells = engine.get_children(cell_query, entity);
        let mut y = 0.0;
        let mut height = data.size.height;
        for (index, cell) in cells.iter().enumerate() {
            let cell_constraint = LayoutConstraint::new(LayoutSize::new(data.size.width, height));
            let cell_size = cell.view.calc_size(engine, cell_constraint);
            cell.set_layout_data(
                layout_query,
                data.new_child(self.pivot(), LayoutAnchor::TOP_LEFT, Vec2::new(0.0, y), cell_size),
            );
            commands.entity(cell.entity).insert(VBoxCellData{
                index,
            });
            y -= cell_size.height;
            height -= cell_size.height;
        }
    }
}
