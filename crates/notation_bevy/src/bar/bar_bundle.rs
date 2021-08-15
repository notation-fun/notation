use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{PlayingState, TabBar};

use crate::prelude::{BarLayout, BarPlaying};

#[derive(Bundle)]
pub struct BarBundle {
    pub bar: Arc<TabBar>,
    pub name: Name,
    pub layout: BarLayout,
    pub playing: BarPlaying,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl BarBundle {
    pub fn new(bar: Arc<TabBar>, layout: BarLayout, transform: Transform) -> Self {
        let name = Name::from(bar.to_string().as_str());
        let playing = BarPlaying::new(&bar, PlayingState::Idle);
        Self {
            bar,
            name,
            layout,
            playing,
            transform,
            global_cransform: GlobalTransform::default(),
        }
    }
}
