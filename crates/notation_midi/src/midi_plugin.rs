use crate::prelude::{MidiHub, MidiSettings, MidiState, PlayControlEvt, SwitchTabEvent};
use bevy::prelude::*;
use notation_model::prelude::PlayClock;

pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SwitchTabEvent>();
        app.add_event::<PlayControlEvt>();
        app.init_resource::<PlayClock>();
        app.init_resource::<MidiSettings>();
        app.init_resource::<MidiState>();
        app.init_non_send_resource::<MidiHub>();
        app.add_system(on_switch_tab.system());
        app.add_system(on_play_control_evt.system());
        app.add_system(do_tick.system());

        #[cfg(not(target_arch = "wasm32"))]
        self.build_native(app);
    }
}

fn on_switch_tab(
    mut _commands: Commands,
    mut evts: EventReader<SwitchTabEvent>,
    settings: Res<MidiSettings>,
    mut hub: NonSendMut<MidiHub>,
    mut state: ResMut<MidiState>,
) {
    for evt in evts.iter() {
        state.switch_tab(&settings, &mut hub, evt.tab.clone());
    }
}

fn on_play_control_evt(
    settings: Res<MidiSettings>,
    mut state: ResMut<MidiState>,
    mut hub: NonSendMut<MidiHub>,
    mut evts: EventReader<PlayControlEvt>,
) {
    for evt in evts.iter() {
        match evt {
            PlayControlEvt::OnPlayState(play_state) => {
                if !play_state.is_playing() {
                    state.init_channels(&settings, &mut hub);
                }
            }
            _ => (),
        }
    }
}

fn do_tick(
    settings: Res<MidiSettings>,
    mut state: ResMut<MidiState>,
    mut hub: NonSendMut<MidiHub>,
    mut clock: ResMut<PlayClock>,
    mut play_control_evts: EventWriter<PlayControlEvt>,
) {
    clock.tick();
    let tick_result = state.tick(&settings, &mut hub, clock.delta_seconds());
    if tick_result.changed {
        play_control_evts.send(PlayControlEvt::on_tick(
            state.play_control.position,
            tick_result,
        ));
    }
}
