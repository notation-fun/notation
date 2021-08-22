use bevy::prelude::*;

use bevy_utils::prelude::{DoLayoutEvent, LyonShapeOp};

use crate::prelude::{
    BarPlaying, NotationTheme,
};
use crate::ui::layout::NotationLayout;

use super::mini_bar::{MiniBar, MiniBarData, MiniBarShape};
use super::mini_map::MiniMap;

pub type MiniMapDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, MiniMap>;


pub struct MiniPlugin;

impl Plugin for MiniPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MiniMapDoLayoutEvent>();
        app.add_system(on_bar_playing_changed.system());
        app.add_system(MiniMap::on_added.system());
        app.add_system(MiniMap::do_layout.system());
        app.add_system(MiniMap::on_layout_changed.system());
        app.add_system(MiniBar::on_added.system());
        app.add_system(MiniBar::on_layout_changed.system());
    }
}

fn on_bar_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut query: Query<(Entity, &BarPlaying, &mut MiniBarData), Changed<BarPlaying>>,
) {
    for (entity, playing, mut data) in query.iter_mut() {
        //println!("{:?} -> {:?} -> {:?}", name, data, playing)
        data.value.playing_state = playing.value;
        MiniBarShape::update(&mut commands, &theme, entity, &data);
    }
}