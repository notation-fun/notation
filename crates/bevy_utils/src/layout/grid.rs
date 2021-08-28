use std::sync::Arc;

use bevy::prelude::*;

use crate::prelude::{
    BevyUtil, LayoutAnchor, LayoutData, LayoutEnv, LayoutQuery, LayoutSize, View, ViewQuery,
};

#[derive(Clone, Debug)]
pub enum GridCellSize {
    Fixed(LayoutSize),
    Rows(Vec<LayoutSize>),
}
impl GridCellSize {
    pub fn calc_cell_offset(&self, margin: &LayoutSize, row: usize, col: usize) -> Vec2 {
        match self {
            Self::Fixed(size) => {
                let x = (size.width + margin.width) * col as f32;
                let y = -(size.height + margin.height) * row as f32;
                Vec2::new(x, y)
            }
            Self::Rows(rows) => {
                let mut y = 0.0;
                for i in 0..row {
                    if let Some(size) = rows.get(i) {
                        y -= size.height + margin.height;
                    } else {
                        break;
                    }
                }
                if let Some(size) = rows.get(row) {
                    let x = (size.width + margin.width) * col as f32;
                    Vec2::new(x, y)
                } else {
                    Vec2::ZERO
                }
            }
        }
    }
    pub fn calc_cell_size(&self, _margin: &LayoutSize, row: usize, _col: usize) -> LayoutSize {
        match self {
            Self::Fixed(size) => size.clone(),
            Self::Rows(rows) => {
                if let Some(size) = rows.get(row) {
                    size.clone()
                } else {
                    LayoutSize::ZERO
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct GridCellData {
    pub grid: Arc<GridData>,
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub struct GridData {
    pub rows: usize,
    pub cols: usize,
    pub size: GridCellSize,
    pub margin: LayoutSize,
    pub offset: Vec2,
    pub grid_size: LayoutSize,
    pub content_size: LayoutSize,
}
impl GridData {
    pub const ZERO: GridData = Self {
        rows: 0,
        cols: 0,
        size: GridCellSize::Fixed(LayoutSize::ZERO),
        margin: LayoutSize::ZERO,
        offset: Vec2::ZERO,
        grid_size: LayoutSize::ZERO,
        content_size: LayoutSize::ZERO,
    };
    pub fn new(
        rows: usize,
        cols: usize,
        size: GridCellSize,
        margin: LayoutSize,
        anchor: LayoutAnchor,
        grid_size: LayoutSize,
        content_size: LayoutSize,
    ) -> Self {
        let space_x = if grid_size.width > content_size.width {
            (grid_size.width - content_size.width) / 2.0
        } else {
            0.0
        };
        let space_y = if grid_size.height > content_size.height {
            -(grid_size.height - content_size.height) / 2.0
        } else {
            0.0
        };
        let offset =
            content_size.calc_offset(LayoutAnchor::TOP_LEFT, anchor) + Vec2::new(space_x, space_y);
        GridData {
            rows,
            cols,
            size,
            margin: margin,
            offset,
            grid_size,
            content_size,
        }
    }
    pub fn new_fixed(
        rows: usize,
        cols: usize,
        size: LayoutSize,
        margin: LayoutSize,
        anchor: LayoutAnchor,
        grid_size: LayoutSize,
    ) -> Self {
        let content_size = Self::calc_fixed_content_size(rows, cols, size, margin);
        Self::new(
            rows,
            cols,
            GridCellSize::Fixed(size),
            margin,
            anchor,
            grid_size,
            content_size,
        )
    }
    pub fn new_rows(
        rows: usize,
        cols: usize,
        row_sizes: Vec<LayoutSize>,
        margin: LayoutSize,
        anchor: LayoutAnchor,
        grid_size: LayoutSize,
    ) -> Self {
        let content_size = Self::calc_rows_content_size(rows, cols, &row_sizes, margin);
        Self::new(
            rows,
            cols,
            GridCellSize::Rows(row_sizes),
            margin,
            anchor,
            grid_size,
            content_size,
        )
    }
    pub fn calc_fixed_content_size(
        rows: usize,
        cols: usize,
        size: LayoutSize,
        margin: LayoutSize,
    ) -> LayoutSize {
        if cols == 0 || rows == 0 {
            return LayoutSize::ZERO;
        }
        let content_width = (size.width + margin.width) * cols as f32 - margin.width;
        let content_height = (size.height + margin.height) * rows as f32 - margin.height;
        LayoutSize::new(content_width, content_height)
    }
    pub fn calc_rows_content_size(
        rows: usize,
        cols: usize,
        row_sizes: &Vec<LayoutSize>,
        margin: LayoutSize,
    ) -> LayoutSize {
        if cols == 0 || rows == 0 {
            return LayoutSize::ZERO;
        }
        let mut content_width = 0.0;
        let mut content_height = -margin.height;
        for row_size in row_sizes {
            let row_content_width = (row_size.width + margin.width) * cols as f32 - margin.width;
            if row_content_width > content_width {
                content_width = row_content_width;
            }
            content_height += row_size.height + margin.height;
        }
        LayoutSize::new(content_width, content_height)
    }
}
impl GridData {
    pub fn cals_fixed_cells_by_side(
        side_size: f32,
        cell_size_range: (f32, f32),
        margin: f32,
        total: usize,
    ) -> (usize, f32) {
        let content_size = side_size - margin * 2.0;
        if total == 0 || content_size <= cell_size_range.0 {
            return (0, 0.0);
        }
        let cell_size_with_margin =
            BevyUtil::in_range_with_margin(content_size / total as f32, cell_size_range, margin);
        let mut cell_count = (content_size / cell_size_with_margin).floor() as usize;
        if cell_count == 0 {
            cell_count = 1;
        }
        let cell_size =
            BevyUtil::in_range(content_size / cell_count as f32 - margin, cell_size_range);
        (cell_count, cell_size)
    }
    pub fn calc_rows(total: usize, cols: usize) -> usize {
        if total == 0 || cols == 0 {
            return 0;
        }
        let mut rows = total / cols;
        if total % cols > 0 {
            rows += 1;
        }
        rows
    }
    pub fn cals_fixed_rows_cols_by_width(
        grid_width: f32,
        cell_width_range: (f32, f32),
        margin_width: f32,
        total: usize,
    ) -> (usize, usize, f32) {
        let (cols, cell_width) =
            Self::cals_fixed_cells_by_side(grid_width, cell_width_range, margin_width, total);
        (Self::calc_rows(total, cols), cols, cell_width)
    }
    pub fn cals_fixed_rows_cols_by_height(
        grid_height: f32,
        cell_height_range: (f32, f32),
        margin_height: f32,
        total: usize,
    ) -> (usize, usize, f32) {
        let (rows, cell_width) =
            Self::cals_fixed_cells_by_side(grid_height, cell_height_range, margin_height, total);
        (rows, Self::calc_rows(total, rows), cell_width)
    }
}
impl GridData {
    pub fn content_size(&self) -> LayoutSize {
        match &self.size {
            GridCellSize::Fixed(size) => {
                Self::calc_fixed_content_size(self.rows, self.cols, *size, self.margin)
            }
            GridCellSize::Rows(rows) => {
                let mut fixed_size = None;
                let mut content_height = self.margin.height;
                for row in rows.iter() {
                    if fixed_size.is_none() {
                        fixed_size = Some(Self::calc_fixed_content_size(
                            self.rows,
                            self.cols,
                            *row,
                            self.margin,
                        ));
                    }
                    content_height += row.height + self.margin.height;
                }
                if fixed_size.is_some() {
                    LayoutSize::new(fixed_size.unwrap().width, content_height)
                } else {
                    LayoutSize::ZERO
                }
            }
        }
    }
    pub fn calc_row_col(&self, index: usize) -> (usize, usize) {
        if self.cols == 0 {
            return (0, 0);
        }
        let mut row = index / self.cols;
        let col = index % self.cols;
        if row > self.rows {
            row = self.rows - 1;
        }
        (row, col)
    }
    pub fn calc_cell_offset(&self, row: usize, col: usize) -> Vec2 {
        self.offset + self.size.calc_cell_offset(&self.margin, row, col)
    }
    pub fn calc_cell_size(&self, row: usize, col: usize) -> LayoutSize {
        self.size.calc_cell_size(&self.margin, row, col)
    }
}

pub trait GridCell<TE: LayoutEnv>: View<TE> {
    fn order(&self) -> usize;
}

pub trait GridView<TE: LayoutEnv, TC: GridCell<TE>>: View<TE> {
    fn sort_cells(&self) -> bool {
        false
    }
    fn calc_grid_data(&self, engine: &TE, grid_size: LayoutSize) -> GridData;
    #[allow(unused_variables)]
    fn calc_row_col(&self, engine: &TE, grid_data: &GridData, index: usize) -> (usize, usize) {
        grid_data.calc_row_col(index)
    }
    #[allow(unused_variables)]
    fn calc_cell_offset(&self, engine: &TE, grid_data: &GridData, row: usize, col: usize) -> Vec2 {
        grid_data.calc_cell_offset(row, col)
    }
    fn calc_cell_size(
        &self,
        _engine: &TE,
        grid_data: &GridData,
        row: usize,
        col: usize,
    ) -> LayoutSize {
        grid_data.calc_cell_size(row, col)
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
        let grid_data = Arc::new(self.calc_grid_data(engine, data.size));
        let mut cells = engine.get_children(cell_query, entity);
        if self.sort_cells() {
            cells.sort_by(|a, b| a.view.order().cmp(&b.view.order()));
        }
        commands.entity(entity).insert(grid_data.clone());
        println!(
            "<GridView<{}>>::do_layout({:?})",
            std::any::type_name::<TC>(),
            grid_data
        );
        for (index, cell) in cells.iter().enumerate() {
            let (row, col) = self.calc_row_col(engine, &grid_data, index);
            let offset = self.calc_cell_offset(engine, &grid_data, row, col);
            let size = self.calc_cell_size(engine, &grid_data, row, col);
            cell.set_layout_data(
                layout_query,
                data.new_child(LayoutAnchor::TOP_LEFT, offset, size),
            );
            commands.entity(cell.entity).insert(GridCellData {
                grid: grid_data.clone(),
                row,
                col,
            });
        }
    }
}
