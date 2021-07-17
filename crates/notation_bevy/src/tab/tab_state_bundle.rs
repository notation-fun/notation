use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Tab;

use crate::config::bevy_config::BevyConfig;

use super::tab_state::TabState;

#[derive(Bundle)]
pub struct TabStateBundle {
    pub tab: Arc<Tab>,
    pub name: Name,
    pub state: TabState,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl TabStateBundle {
    pub fn new(config: &BevyConfig, tab: Arc<Tab>) -> Self {
        let name = format!("State: {}", tab).as_str().into();
        let state = TabState::new(&tab);
        let transform = config.grid.calc_tab_transform();
        Self {
            tab,
            name,
            state,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
