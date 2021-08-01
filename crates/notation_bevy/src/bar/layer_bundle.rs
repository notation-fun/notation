use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{BarLayer, TabBar};

#[derive(Bundle)]
pub struct LayerBundle {
    pub bar: Arc<TabBar>,
    pub layer: Arc<BarLayer>,
    pub name: Name,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl LayerBundle {
    pub fn new(bar: Arc<TabBar>, layer: Arc<BarLayer>) -> Self {
        let name = format!("{} {}", bar.bar_ordinal, layer).as_str().into();
        Self {
            bar,
            layer,
            name,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
