use bevy::prelude::*;

use crate::prelude::{
    LayoutAnchor, LayoutData, LayoutEnv, LayoutQuery, LayoutSize, View, ViewQuery,
};

#[derive(Copy, Clone, Debug)]
pub enum GridCellSize {
    Fixed(LayoutSize),
}
impl GridCellSize {
    pub fn calc_cell_offset(&self, row: usize, col: usize) -> Vec2 {
        match self {
            Self::Fixed(size) => {
                let x = size.width * col as f32;
                let y = -size.height * row as f32;
                Vec2::new(x, y)
            },
        }
    }
    pub fn calc_cell_size(&self, _row: usize, _col: usize) -> LayoutSize {
        match self {
            Self::Fixed(size) => size.clone(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GridData {
    pub rows: usize,
    pub cols: usize,
    pub size: GridCellSize,
    pub offset: Vec2,
}
impl GridData {
    pub const ZERO: GridData = Self {
        rows: 0,
        cols: 0,
        size: GridCellSize::Fixed(LayoutSize{width: 0.0, height: 0.0}),
        offset: Vec2::ZERO,
    };
    pub fn calc_rows(total: usize, cols: usize) -> usize {
        if total == 0 || cols == 0 {
            return 0;
        }
        let mut rows = total / cols;
        if total % cols == 0 {
            rows += 1;
        }
        rows
    }
    pub fn calc_row_col(&self, index: usize) -> (usize, usize) {
        let mut row = index / self.cols;
        let col = index % self.cols;
        if row > self.rows {
            row = self.rows - 1;
        }
        (row, col)
    }
    pub fn calc_cell_offset(&self, row: usize, col: usize) -> Vec2 {
        self.offset + self.size.calc_cell_offset(row, col)
    }
    pub fn calc_cell_size(&self, row: usize, col: usize) -> LayoutSize {
        self.size.calc_cell_size(row, col)
    }
}

pub trait GridCell<TE: LayoutEnv>: View<TE> {}

pub trait GridView<TE: LayoutEnv, TC: GridCell<TE>>: View<TE> {
    fn calc_grid_data(&self, _engine: &TE, data: LayoutData) -> GridData;
    fn calc_row_col(&self, _engine: &TE, grid_data: &GridData, index: usize) -> (usize, usize) {
        grid_data.calc_row_col(index)
    }
    fn calc_cell_offset(&self, _engine: &TE, grid_data: &GridData, row: usize, col: usize) -> Vec2 {
        grid_data.calc_cell_offset(row, col)
    }
    fn calc_cell_size(&self, _engine: &TE, grid_data: &GridData, row: usize, col: usize) -> LayoutSize {
        grid_data.calc_cell_size(row, col)
    }
    fn do_layout(
        &self,
        engine: &TE,
        layout_query: &mut LayoutQuery,
        cell_query: &ViewQuery<TC>,
        entity: Entity,
        data: LayoutData,
    ) {
        if self.is_root() {
            self.set_layout_data(layout_query, entity, data);
        }
        let grid_data = self.calc_grid_data(engine, data);
        let cells = engine.get_children(cell_query, entity);
        for (index, cell) in cells.iter().enumerate() {
            let (row, col) = self.calc_row_col(engine, &grid_data, index);
            let offset = self.calc_cell_offset(engine, &grid_data, row, col);
            let size = self.calc_cell_size(engine, &grid_data, row, col);
            cell.set_layout_data(
                layout_query,
                data.new_child(self.pivot(), LayoutAnchor::TOP_LEFT, offset, size),
            );
        }
    }
}
