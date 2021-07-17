use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{TabBar, TabPosition};

use crate::prelude::{GridCol, GridConfig, GridRow};

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
    pub fn new(bar: Arc<TabBar>, config: &GridConfig) -> Self {
        let (row, col) = config.calc_bar_row_col(&bar);
        let transform = config.calc_bar_transform(&row, &col);
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
