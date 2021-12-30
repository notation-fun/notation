use bevy::prelude::*;

use crate::prelude::NotationAssetsStates;
use crate::tab::tab_events::BarViewDoLayoutEvent;

use super::bar_view::BarView;

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        BarViewDoLayoutEvent::setup(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(BarView::do_layout.system())
                .with_system(BarView::update_number_text.system()),
        );
    }
}
