use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{BarLane, TabBar};

#[derive(Bundle)]
pub struct LaneBundle {
    pub bar: Arc<TabBar>,
    pub lane: Arc<BarLane>,
    pub name: Name,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl LaneBundle {
    pub fn new(bar: Arc<TabBar>, lane: Arc<BarLane>) -> Self {
        let name = format!("{} {}", bar.bar_ordinal, lane).as_str().into();
        Self {
            bar,
            lane,
            name,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
