use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Tab;

use crate::config::bevy_config::BevyConfig;

#[derive(Bundle)]
pub struct TabBundle {
    pub tab: Arc<Tab>,
    pub name: Name,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl TabBundle {
    pub fn new(config: &BevyConfig, v: Arc<Tab>) -> Self {
        let name = v.to_string().as_str().into();
        let transform = config.grid.calc_tab_transform();
        Self {
            tab: v,
            name,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
