pub mod midi_synth;
pub mod embedded_api;

use bevy::prelude::*;
use notation_audio::prelude::StereoStream;

use crate::prelude::{MidiPlugin, MidiHub};

impl MidiPlugin {
    pub fn build_native(&self, app: &mut App) {
        StereoStream::init_streaming(app, true);
        app.add_systems(Update, send_synth_buffer);
    }
}

fn send_synth_buffer(
    mut hub: NonSendMut<MidiHub>,
    mut stream: ResMut<StereoStream>,
) {
    hub.send_buffer(&mut stream);
}

impl MidiHub {
    pub fn send_buffer(&mut self, stream: &mut StereoStream) {
        if let Some(synth) = self.output_synth.as_mut() {
            synth.send_buffer(stream);
        }
    }
}

/*
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
 */
