use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{TabBar, TabPosition};

use crate::prelude::{BarLayout};

#[derive(Bundle)]
pub struct BarBundle {
    pub bar: Arc<TabBar>,
    pub name: Name,
    pub pos: TabPosition,
    pub layout: BarLayout,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl BarBundle {
    pub fn new(bar: Arc<TabBar>, layout: BarLayout, transform: Transform) -> Self {
        let name = Name::from(bar.to_string().as_str());
        let pos = bar.tab_pos();
        Self {
            bar,
            name,
            pos,
            layout,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
