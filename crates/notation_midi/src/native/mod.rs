pub mod audio_stream;
pub mod midi_synth;

use bevy::prelude::*;
use bevy_kira_audio::{AudioPlugin, AudioStreamPlugin, StreamedAudio};

use audio_stream::{DoubleAudioBuffer, MidiAudioStream};
use crate::prelude::{MidiPlugin, MidiHub, MidiSettings};

impl MidiPlugin {
    pub fn build_native(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin);
        app.add_plugin(AudioStreamPlugin::<MidiAudioStream>::default());
        app.add_startup_system(setup_audio_stream.system());
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

impl MidiHub {
    pub fn get_synth_buffer(&mut self, settings: &MidiSettings) -> Option<DoubleAudioBuffer> {
        self.check_output(settings);
        if let Some(synth) = &self.output_synth {
            synth.get_buffer()
        } else {
            None
        }
    }
    pub fn check_synth_buffer(&mut self) {
        if let Some(synth) = self.output_synth.as_mut() {
            synth.check_buffer();
        }
    }
}