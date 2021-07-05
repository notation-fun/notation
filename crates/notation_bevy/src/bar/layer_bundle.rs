use bevy::prelude::*;
use std::sync::Arc;

use notation_proto::prelude::{BarLayer, TabBar};

#[derive(Bundle)]
pub struct LayerBundle {
    pub tab: Arc<BarLayer>,
    pub name: Name,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl LayerBundle {
    pub fn new(bar: &TabBar, layer: Arc<BarLayer>) -> Self {
        let name = format!("{} {}", bar.bar_ordinal, layer).as_str().into();
        Self {
            tab: layer,
            name,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
