use helgoboss_midi::StructuredShortMessage;

use crate::prelude::{DoubleAudioBuffer};

pub struct MidiSynth {
    synth: fluidlite::Synth,
    buffer: DoubleAudioBuffer,
}
impl MidiSynth {
    fn new(synth: fluidlite::Synth) -> Self {
        Self {
            synth,
            buffer: DoubleAudioBuffer::new(),
        }
    }
    pub fn try_new() -> Option<MidiSynth> {
        fluidlite::Settings::new()
            .and_then(fluidlite::Synth::new)
            .and_then(|synth| {
                synth.sfload("assets/gm.sf2", true).map(|_| synth)
            }).map(Self::new)
            .map_err(|err| {
                println!("MidiSynth try_new() failed: {:?}", err);
                err
            }).ok()
    }
    pub fn get_buffer(&self) -> DoubleAudioBuffer {
        self.buffer.clone()
    }
    pub fn check_buffer(&mut self) {
        let synth = &self.synth;
        self.buffer.write_buffer(|mut data| {
            println!("NativeMidiSynth writing buffer: [{}]", data.len());
            synth.write(data).unwrap();
        });
    }
    pub fn send(&self, msg: StructuredShortMessage) -> Result<(), String> {
        match msg {
            StructuredShortMessage::NoteOff { channel, key_number, velocity } =>
                self.synth.note_off(channel.into(), key_number.into()),
            StructuredShortMessage::NoteOn { channel, key_number, velocity } =>
                self.synth.note_on(channel.into(), key_number.into(), velocity.into()),
            StructuredShortMessage::PolyphonicKeyPressure { channel, key_number, pressure_amount } => todo!(),
            StructuredShortMessage::ControlChange { channel, controller_number, control_value } => todo!(),
            StructuredShortMessage::ProgramChange { channel, program_number } => todo!(),
            StructuredShortMessage::ChannelPressure { channel, pressure_amount } => todo!(),
            StructuredShortMessage::PitchBendChange { channel, pitch_bend_value } => todo!(),
            StructuredShortMessage::SystemExclusiveStart => todo!(),
            StructuredShortMessage::TimeCodeQuarterFrame(_) => todo!(),
            StructuredShortMessage::SongPositionPointer { position } => todo!(),
            StructuredShortMessage::SongSelect { song_number } => todo!(),
            StructuredShortMessage::TuneRequest => todo!(),
            StructuredShortMessage::SystemExclusiveEnd => todo!(),
            StructuredShortMessage::TimingClock => todo!(),
            StructuredShortMessage::Start => todo!(),
            StructuredShortMessage::Continue => todo!(),
            StructuredShortMessage::Stop => todo!(),
            StructuredShortMessage::ActiveSensing => todo!(),
            StructuredShortMessage::SystemReset => todo!(),
            StructuredShortMessage::SystemCommonUndefined1 => todo!(),
            StructuredShortMessage::SystemCommonUndefined2 => todo!(),
            StructuredShortMessage::SystemRealTimeUndefined1 => todo!(),
            StructuredShortMessage::SystemRealTimeUndefined2 => todo!(),
        }.map_err(|err| {
            format!("{:?}", err)
        })
    }
}


