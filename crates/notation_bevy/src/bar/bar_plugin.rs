use bevy::prelude::*;

use crate::tab::tab_events::BarViewDoLayoutEvent;

use super::bar_view::BarView;

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        BarViewDoLayoutEvent::setup(app);
        app.add_system(BarView::on_added.system());
        app.add_system(BarView::do_layout.system());
    }
}

