use crate::prelude::{
    AddToneEvent, MidiAudioStream, MidiHub, MidiSettings, MidiState, PlayControlEvt, PlayToneEvent,
    StopToneEvent, SwitchTabEvent,
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioPlugin, AudioStreamPlugin, StreamedAudio};
use notation_model::{play::{self, play_control::TickResult}, prelude::{PlayClock, PlayState}};
pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin);
        app.add_plugin(AudioStreamPlugin::<MidiAudioStream>::default());
        app.add_event::<SwitchTabEvent>();
        app.add_event::<AddToneEvent>();
        app.add_event::<PlayControlEvt>();
        app.add_event::<PlayToneEvent>();
        app.add_event::<StopToneEvent>();
        app.init_resource::<PlayClock>();
        app.init_resource::<MidiSettings>();
        app.init_resource::<MidiState>();
        app.init_non_send_resource::<MidiHub>();
        app.add_startup_system(setup_audio_stream.system());
        app.add_system(on_switch_tab.system());
        app.add_system(on_add_tone.system());
        app.add_system(on_play_tone.system());
        app.add_system(on_stop_tone.system());
        app.add_system(check_synth_buffer.system());
        app.add_system(on_play_control_evt.system());
        app.add_system(do_tick.system());
    }
}

fn setup_audio_stream(
    streamed_audio: Res<StreamedAudio<MidiAudioStream>>,
    mut hub: NonSendMut<MidiHub>,
    settings: Res<MidiSettings>,
) {
    if let Some(buffer) = hub.get_synth_buffer(&settings) {
        let audio_stream = MidiAudioStream::new(buffer);
        streamed_audio.stream(audio_stream);
    }
}

fn check_synth_buffer(mut hub: NonSendMut<MidiHub>) {
    hub.check_synth_buffer();
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

fn on_add_tone(
    mut _commands: Commands,
    mut evts: EventReader<AddToneEvent>,
    mut state: ResMut<MidiState>,
) {
    for evt in evts.iter() {
        state.add_tone(evt);
    }
}

fn on_play_tone(
    mut _commands: Commands,
    mut evts: EventReader<PlayToneEvent>,
    settings: Res<MidiSettings>,
    mut hub: NonSendMut<MidiHub>,
    state: Res<MidiState>,
) {
    for evt in evts.iter() {
        if let Some(channel) = state.get_channel(&evt.track_id, &evt.track_kind) {
            for msg in evt.to_midi_msgs(channel) {
                hub.send(&settings, msg);
            }
        }
    }
}

fn on_stop_tone(
    mut _commands: Commands,
    mut evts: EventReader<StopToneEvent>,
    settings: Res<MidiSettings>,
    state: Res<MidiState>,
    mut hub: NonSendMut<MidiHub>,
) {
    for evt in evts.iter() {
        if let Some(channel) = state.get_channel(&evt.track_id, &evt.track_kind) {
            for msg in evt.to_midi_msgs(channel) {
                hub.send(&settings, msg);
            }
        }
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

fn do_tick (
    settings: Res<MidiSettings>,
    mut state: ResMut<MidiState>,
    mut hub: NonSendMut<MidiHub>,
    mut clock: ResMut<PlayClock>,
    mut play_control_evts: EventWriter<PlayControlEvt>,
) {
    clock.tick();
    let tick_result = state.tick(&settings, &mut hub, clock.delta_seconds());
    if tick_result.changed {
        play_control_evts.send(PlayControlEvt::on_tick(state.play_control.position, tick_result));
    }
}