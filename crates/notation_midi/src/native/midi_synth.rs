use helgoboss_midi::StructuredShortMessage;

use crate::prelude::MidiMessage;
use notation_model::prelude::PlaySpeed;

use super::audio_stream::DoubleAudioBuffer;

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
            .and_then(|synth| synth.sfload("assets/sf2/SBLive.sf2", true).map(|_| synth))
            .map(Self::new)
            .map_err(|err| {
                println!("MidiSynth try_new() failed: {:?}", err);
                err
            })
            .ok()
    }
    pub fn get_buffer(&self) -> Option<DoubleAudioBuffer> {
        Some(self.buffer.clone())
    }
    pub fn check_buffer(&mut self) {
        let synth = &self.synth;
        // let use_buffer_2 = self.buffer.use_buffer_2;
        self.buffer.write_buffer(|data| {
            /*
            println!("NativeMidiSynth writing buffer: {} [{}]",
                if use_buffer_2 { 2 } else { 1 },
                data.len());
             */
            synth.write(data).unwrap();
        });
    }
    pub fn send(&self, _speed: &PlaySpeed, msg: &MidiMessage) -> Result<(), String> {
        match msg.midi {
            StructuredShortMessage::NoteOff {
                channel,
                key_number,
                velocity: _,
            } => self.synth.note_off(channel.into(), key_number.into()),
            StructuredShortMessage::NoteOn {
                channel,
                key_number,
                velocity,
            } => self
                .synth
                .note_on(channel.into(), key_number.into(), velocity.into()),
            StructuredShortMessage::PolyphonicKeyPressure {
                channel: _,
                key_number: _,
                pressure_amount: _,
            } => todo!(),
            StructuredShortMessage::ControlChange {
                channel,
                controller_number,
                control_value,
            } => self.synth.cc(
                channel.into(),
                controller_number.into(),
                control_value.into(),
            ),
            StructuredShortMessage::ProgramChange {
                channel,
                program_number,
            } => self
                .synth
                .program_change(channel.into(), program_number.into()),
            StructuredShortMessage::ChannelPressure {
                channel: _,
                pressure_amount: _,
            } => todo!(),
            StructuredShortMessage::PitchBendChange {
                channel: _,
                pitch_bend_value: _,
            } => todo!(),
            StructuredShortMessage::SystemExclusiveStart => todo!(),
            StructuredShortMessage::TimeCodeQuarterFrame(_) => todo!(),
            StructuredShortMessage::SongPositionPointer { position: _ } => todo!(),
            StructuredShortMessage::SongSelect { song_number: _ } => todo!(),
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
        }
        .map_err(|err| format!("{:?}", err))
    }
}
