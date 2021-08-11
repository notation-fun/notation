use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Tab;

use crate::prelude::BarLayout;

use super::tab_state::TabState;

#[derive(Bundle, Debug)]
pub struct TabStateBundle {
    pub bar_layouts: Arc<Vec<BarLayout>>,
    pub name: Name,
    pub state: TabState,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl TabStateBundle {
    pub fn new(tab: Arc<Tab>, bar_layouts: Arc<Vec<BarLayout>>) -> Self {
        let name = format!("State: {}", tab).as_str().into();
        let state = TabState::new(&tab);
        Self {
            bar_layouts,
            name,
            state,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
