use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{TabBar, TabPosition};

use crate::prelude::{GridCol, GridRow, NotationSettings, NotationTheme};

#[derive(Bundle)]
pub struct BarBundle {
    pub bar: Arc<TabBar>,
    pub name: Name,
    pub pos: TabPosition,
    pub row: GridRow,
    pub col: GridCol,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl BarBundle {
    pub fn new(settings: &NotationSettings, theme: &NotationTheme, bar: Arc<TabBar>) -> Self {
        let (row, col) = theme.grid.calc_bar_row_col(settings, &bar);
        let transform = theme.grid.calc_bar_transform(&row, &col);
        let name = Name::from(bar.to_string().as_str());
        let pos = bar.tab_pos();
        Self {
            bar,
            name,
            pos,
            row,
            col,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
