use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::prelude::LayoutData;
use std::sync::Arc;

use notation_model::prelude::TabBar;
use notation_midi::prelude::PlayingState;

use crate::prelude::{BarLayoutData, BarPlaying};

use super::bar_view::BarView;

#[derive(Bundle)]
pub struct BarBundle {
    //pub bar: Arc<TabBar>,
    pub name: Name,
    pub view: BarView,
    pub layout: LayoutData,
    pub bar_layout: BarLayoutData,
    pub playing: BarPlaying,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl BarBundle {
    pub fn new(bar: Arc<TabBar>, bar_layout: BarLayoutData) -> Self {
        let name = Name::from(bar.to_string().as_str());
        let view = BarView::new(&bar, bar_layout.clone());
        let playing = BarPlaying::new(&bar, PlayingState::Idle);
        Self {
            //bar,
            name,
            view,
            layout: LayoutData::ZERO,
            bar_layout,
            playing,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}
