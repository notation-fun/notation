use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{TabBar, Units};

use crate::prelude::{GridCol, GridConfig, GridRow};

#[derive(Bundle)]
pub struct BarBundle {
    pub bar: Arc<TabBar>,
    pub name: Name,
    pub length: Units,
    pub row: GridRow,
    pub col: GridCol,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl BarBundle {
    pub fn new(bar: Arc<TabBar>, config: &GridConfig) -> Self {
        let (row, col) = config.calc_bar_row_col(&bar);
        let transform = config.calc_bar_transform(bar.bar_units(), &row, &col);
        let name = Name::from(bar.to_string().as_str());
        Self {
            bar,
            name,
            length: Units(0.0),
            row,
            col,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
