use edger_bevy::bevy_prelude::*;

use edger_bevy::prelude::AssetsStates;

use crate::tab::tab_events::BarViewDoLayoutEvent;

use super::bar_view::BarView;

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        BarViewDoLayoutEvent::setup(app);
        app.add_systems(Update, (
            BarView::do_layout,
            BarView::update_number_text,
        ).run_if(in_state(AssetsStates::Loaded)));
    }
}
