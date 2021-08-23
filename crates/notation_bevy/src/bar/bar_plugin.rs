use bevy::prelude::*;

use super::bar_view::BarView;

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(BarView::on_added.system());
        app.add_system(BarView::do_layout.system());
        app.add_system(BarView::on_layout_changed.system());
    }
}

