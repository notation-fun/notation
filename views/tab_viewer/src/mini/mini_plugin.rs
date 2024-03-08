use edger_bevy::bevy_prelude::*;

use edger_bevy::prelude::{DoLayoutEvent, ShapeOp, AssetsStates};

use crate::prelude::{BarPlaying, NotationTheme};
use crate::prelude::NotationLayout;

use super::mini_bar::{MiniBar, MiniBarData};
use super::mini_map::MiniMap;

pub type MiniMapDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, MiniMap>;

pub struct MiniPlugin;

impl Plugin for MiniPlugin {
    fn build(&self, app: &mut App) {
        MiniMapDoLayoutEvent::setup(app);
        app.add_systems(Update, (
            on_bar_playing_changed,
            MiniMap::do_layout,
            MiniMap::update_debug_str,
            MiniBar::on_layout_changed,
        ).run_if(in_state(AssetsStates::Loaded)));
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
        data.update(&mut commands, &theme, entity);
    }
}
