use bevy::prelude::*;
use std::sync::Arc;

use notation_proto::prelude::{TabBar, Units};

use crate::prelude::{GridCol, GridConfig, GridRow};

#[derive(Bundle)]
pub struct BarBundle {
    pub tab: Arc<TabBar>,
    pub name: Name,
    pub length: Units,
    pub row: GridRow,
    pub col: GridCol,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl BarBundle {
    pub fn new(bar: Arc<TabBar>, config: &GridConfig) -> Self {
        let ordinal_ = bar.bar_ordinal - 1;
        let row = GridRow(ordinal_ / config.bars_in_row as usize);
        let col = GridCol(ordinal_ % config.bars_in_row as usize);
        let x = config.unit_size * bar.units().0 * col.0 as f32;
        let y = config.semitone_size * 18.0 * row.0 as f32;
        let name = Name::from(bar.to_string().as_str());
        Self {
            tab: bar,
            name,
            length: Units(0.0),
            row,
            col,
            transform: Transform::from_xyz(x, y, 0.0),
            global_cransform: GlobalTransform::default(),
        }
    }
}
