use helgoboss_midi::StructuredShortMessage;

use crate::prelude::DoubleAudioBuffer;

pub struct MidiSynth {}

impl MidiSynth {
    pub fn try_new() -> Option<MidiSynth> {
        None
    }
    pub fn send(&self, msg: StructuredShortMessage) -> Result<(), String> {
        todo!()
    }
    pub fn get_buffer(&self) -> Option<DoubleAudioBuffer> {
        None
    }
    pub fn check_buffer(&mut self) {
    }
}
