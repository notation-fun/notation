use bevy::prelude::*;
use std::sync::Arc;

use notation_proto::prelude::{Tab, Units};

use crate::config::bevy_config::BevyConfig;

#[derive(Bundle)]
pub struct TabBundle {
    pub tab: Arc<Tab>,
    pub name: Name,
    pub length: Units,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl TabBundle {
    pub fn new(config: &BevyConfig, v: Arc<Tab>) -> Self {
        let name = v.to_string().as_str().into();
        let transform = config.grid.calc_tab_transform(&v.meta.signature);
        Self {
            tab: v,
            name,
            length: Units(0.0),
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
