use crate::prelude::{
    MidiHub, MidiSettings, MidiState,
};
use bevy::prelude::*;
use crate::prelude::{
    PlayClock, JumpToBarEvent, PlayControlEvent, SwitchTabEvent,
};

#[derive(Default, Resource)]
pub struct MidiClock {
    pub clock: PlayClock,
}

pub struct MidiPlugin;

const DO_TICK_TIMESTEP: f64 = 1.0 / 60.0;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MidiClock>();
        app.init_resource::<MidiSettings>();
        app.init_resource::<MidiState>();
        app.init_non_send_resource::<MidiHub>();
        app.add_systems(Update, on_switch_tab);
        app.add_systems(Update, on_jump_to_bar);
        app.add_systems(Update, on_play_control_evt);
        app.insert_resource(Time::<Fixed>::from_seconds(DO_TICK_TIMESTEP));
        app.add_systems(FixedUpdate, do_tick);
        #[cfg(not(target_arch = "wasm32"))]
        self.build_native(app);
    }
}

fn on_switch_tab(
    mut evts: EventReader<SwitchTabEvent>,
    settings: Res<MidiSettings>,
    mut state: ResMut<MidiState>,
    mut hub: NonSendMut<MidiHub>,
    mut play_control_evts: EventWriter<PlayControlEvent>,
) {
    for evt in evts.read() {
        hub.switch_tab(&settings, &mut state, evt.tab.clone());
        _do_tick(
            &settings,
            &mut state,
            &mut hub,
            &mut play_control_evts,
            true,
            0.0,
        );
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
    for evt in evts.read() {
        bar_props = Some(evt.bar_props);
    }
    if let Some(bar_props) = bar_props {
        state.jump_to_bar(&settings, &mut hub, bar_props);
        _do_tick(
            &settings,
            &mut state,
            &mut hub,
            &mut play_control_evts,
            true,
            0.0,
        );
    }
}

fn on_play_control_evt(
    settings: Res<MidiSettings>,
    mut state: ResMut<MidiState>,
    mut hub: NonSendMut<MidiHub>,
    mut evts: EventReader<PlayControlEvent>,
) {
    for evt in evts.read() {
        match evt {
            PlayControlEvent::OnPlayState(play_state) => {
                state.seek_position = None;
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
    jumped: bool,
    delta_seconds: f32,
) {
    let tick_result = state.tick(settings, hub, jumped, delta_seconds);
    if jumped || tick_result.changed {
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
    mut clock: ResMut<MidiClock>,
    mut play_control_evts: EventWriter<PlayControlEvent>,
) {
    clock.clock.tick();
    //println!("do_tick() -> {}", clock.delta_seconds());
    _do_tick(
        &settings,
        &mut state,
        &mut hub,
        &mut play_control_evts,
        false,
        clock.clock.delta_seconds(),
    );
}
