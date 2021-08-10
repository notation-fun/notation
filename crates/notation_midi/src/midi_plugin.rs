use crate::prelude::{
    MidiAudioStream, MidiHub, MidiSettings, MidiState, PlayToneEvent, StopToneEvent, SwitchTabEvent
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioPlugin, AudioStreamPlugin, StreamedAudio};
pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin);
        app.add_plugin(AudioStreamPlugin::<MidiAudioStream>::default());
        app.add_event::<SwitchTabEvent>();
        app.add_event::<PlayToneEvent>();
        app.add_event::<StopToneEvent>();
        app.init_resource::<MidiSettings>();
        app.init_resource::<MidiState>();
        app.init_non_send_resource::<MidiHub>();
        app.add_startup_system(setup_audio_stream.system());
        app.add_system(on_switch_tab.system());
        app.add_system(on_play_tone.system());
        app.add_system(on_stop_tone.system());
        app.add_system(check_synth_buffer.system());
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
        state.switch_tab(&settings, evt.tab.clone());
        hub.setup_channels(&settings, &state);
    }
}

fn on_play_tone(
    mut _commands: Commands,
    mut evts: EventReader<PlayToneEvent>,
    mut hub: NonSendMut<MidiHub>,
    settings: Res<MidiSettings>,
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
    mut hub: NonSendMut<MidiHub>,
    settings: Res<MidiSettings>,
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
