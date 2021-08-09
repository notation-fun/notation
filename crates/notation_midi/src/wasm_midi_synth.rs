use helgoboss_midi::StructuredShortMessage;

pub struct MidiSynth {
}
impl MidiSynth {
    pub fn try_new() -> Option<MidiSynth> {
        None;
    }
    pub fn send(&self, msg: StructuredShortMessage) -> Result<(), String> {
        todo!()
    }
}