use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Tab;

use crate::prelude::{NotationSettings, NotationTheme};

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
    pub fn new(settings: &NotationSettings, theme: &NotationTheme, tab: Arc<Tab>) -> Self {
        let name = format!("State: {}", tab).as_str().into();
        let state = TabState::new(&tab);
        let transform = theme.grid.calc_tab_transform(settings);
        Self {
            tab,
            name,
            state,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
