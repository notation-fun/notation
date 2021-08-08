use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{BarLane, TabBar};

use crate::prelude::LaneLayout;

#[derive(Bundle)]
pub struct LaneBundle {
    pub name: Name,
    pub bar: Arc<TabBar>,
    pub lane: Arc<BarLane>,
    pub layout: LaneLayout,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl LaneBundle {
    pub fn new(bar: Arc<TabBar>, lane: Arc<BarLane>, layout: LaneLayout) -> Self {
        let name = format!("{} {}", bar.bar_ordinal, lane).as_str().into();
        Self {
            name,
            bar,
            lane,
            layout,
            transform: layout.calc_transform(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
