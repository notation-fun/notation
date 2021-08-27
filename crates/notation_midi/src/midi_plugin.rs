use crate::prelude::{JumpToBarEvent, MidiHub, MidiSettings, MidiState, PlayControlEvent, SwitchTabEvent};
use bevy::prelude::*;
use notation_model::prelude::PlayClock;

pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SwitchTabEvent>();
        app.add_event::<JumpToBarEvent>();
        app.add_event::<PlayControlEvent>();
        app.init_resource::<PlayClock>();
        app.init_resource::<MidiSettings>();
        app.init_resource::<MidiState>();
        app.init_non_send_resource::<MidiHub>();
        app.add_system(on_switch_tab.system());
        app.add_system(on_jump_to_bar.system());
        app.add_system(on_play_control_evt.system());
        app.add_system(do_tick.system());

        #[cfg(not(target_arch = "wasm32"))]
        self.build_native(app);
    }
}

fn on_switch_tab(
    mut evts: EventReader<SwitchTabEvent>,
    settings: Res<MidiSettings>,
    mut hub: NonSendMut<MidiHub>,
    mut state: ResMut<MidiState>,
) {
    for evt in evts.iter() {
        state.switch_tab(&settings, &mut hub, evt.tab.clone());
    }
}

fn on_jump_to_bar(
    mut evts: EventReader<JumpToBarEvent>,
    settings: Res<MidiSettings>,
    mut state: ResMut<MidiState>,
    mut hub: NonSendMut<MidiHub>,
    mut play_control_evts: EventWriter<PlayControlEvent>,
) {
    let mut bar_props = None;
    for evt in evts.iter() {
        bar_props = Some(evt.bar_props);
    }
    if let Some(bar_props) = bar_props {
        state.jump_to_bar(bar_props);
        _do_tick(&settings, &mut state, &mut hub, &mut play_control_evts, true, 0.0);
    }
}

fn on_play_control_evt(
    settings: Res<MidiSettings>,
    mut state: ResMut<MidiState>,
    mut hub: NonSendMut<MidiHub>,
    mut evts: EventReader<PlayControlEvent>,
) {
    for evt in evts.iter() {
        match evt {
            PlayControlEvent::OnPlayState(play_state) => {
                if !play_state.is_playing() {
                    state.init_channels(&settings, &mut hub);
                }
            }
            _ => (),
        }
    }
}

fn _do_tick(
    settings: &MidiSettings,
    state: &mut MidiState,
    hub: &mut MidiHub,
    play_control_evts: &mut EventWriter<PlayControlEvent>,
    send_unchanged: bool,
    delta_seconds: f32,
) {
    let tick_result = state.tick(settings, hub, delta_seconds);
    if send_unchanged || tick_result.changed {
        play_control_evts.send(PlayControlEvent::on_tick(
            state.play_control.position,
            tick_result,
        ));
    }
}

fn do_tick(
    settings: Res<MidiSettings>,
    mut state: ResMut<MidiState>,
    mut hub: NonSendMut<MidiHub>,
    mut clock: ResMut<PlayClock>,
    mut play_control_evts: EventWriter<PlayControlEvent>,
) {
    clock.tick();
    _do_tick(&settings, &mut state, &mut hub, &mut play_control_evts, false, clock.delta_seconds());
}
