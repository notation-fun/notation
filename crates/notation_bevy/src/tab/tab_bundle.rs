use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Tab;

use crate::prelude::{NotationAppState, NotationSettings, NotationTheme};

#[derive(Bundle)]
pub struct TabBundle {
    pub tab: Arc<Tab>,
    pub name: Name,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl TabBundle {
    pub fn new(
        app_state: &NotationAppState,
        settings: &NotationSettings,
        theme: &NotationTheme,
        v: Arc<Tab>,
    ) -> Self {
        let name = v.to_string().as_str().into();
        let transform = theme.grid.calc_tab_transform(app_state, settings);
        Self {
            tab: v,
            name,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
