use bevy::prelude::*;
use bevy_utils::prelude::LayoutData;
use std::sync::Arc;

use notation_model::prelude::{PlayingState, TabBar};

use crate::prelude::{BarLayoutData, BarPlaying};

use super::bar_view::BarView;

#[derive(Bundle)]
pub struct BarBundle {
    pub bar: Arc<TabBar>,
    pub name: Name,
    pub view: Arc<BarView>,
    pub layout: LayoutData,
    pub bar_layout: BarLayoutData,
    pub playing: BarPlaying,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl BarBundle {
    pub fn new(bar: Arc<TabBar>, bar_layout: BarLayoutData) -> Self {
        let name = Name::from(bar.to_string().as_str());
        let view = Arc::new(BarView::new(&bar, bar_layout.clone()));
        let playing = BarPlaying::new(&bar, PlayingState::Idle);
        Self {
            bar,
            name,
            view,
            layout: LayoutData::ZERO,
            bar_layout,
            playing,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
