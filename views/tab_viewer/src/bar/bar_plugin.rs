use bevy::prelude::*;

use crate::prelude::NotationAssetsStates;
use crate::tab::tab_events::BarViewDoLayoutEvent;

use super::bar_view::BarView;

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        BarViewDoLayoutEvent::setup(app);
        app.add_systems((
            BarView::do_layout,
            BarView::update_number_text,
        ).in_set(OnUpdate(NotationAssetsStates::Loaded)));
    }
}
