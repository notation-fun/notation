use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Tab;

use crate::prelude::BarLayout;

use super::tab_state::TabState;

#[derive(Bundle, Debug)]
pub struct TabBundle {
    pub name: Name,
    pub tab: Arc<Tab>,
    pub bar_layouts: Arc<Vec<BarLayout>>,
    pub state: TabState,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl TabBundle {
    pub fn new(tab: Arc<Tab>, bar_layouts: Arc<Vec<BarLayout>>, transform: Transform) -> Self {
        let name = tab.to_string().as_str().into();
        let state = TabState::new(&tab);
        Self {
            name,
            tab,
            bar_layouts,
            state,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
