use std::convert::TryFrom;

use helgoboss_midi::{Channel, KeyNumber, StructuredShortMessage, U7};
use notation_proto::prelude::{Note, Semitones, Tone};

pub struct MidiUtil();

impl MidiUtil {
    pub fn note_midi_key_number(note: &Note) -> Option<KeyNumber> {
        let midi_note = Semitones::from(*note).0 + 12;
        KeyNumber::try_from(midi_note as u8).ok()
    }
    pub fn note_midi_on_msg(note: &Note) -> Option<StructuredShortMessage> {
        Self::note_midi_key_number(note).map(|key_number| -> StructuredShortMessage {
            StructuredShortMessage::NoteOn {
                channel: Channel::new(1),
                key_number,
                velocity: U7::new(127),
            }
        })
    }
    pub fn note_midi_off_msg(note: &Note) -> Option<StructuredShortMessage> {
        Self::note_midi_key_number(note).map(|key_number| StructuredShortMessage::NoteOff {
            channel: Channel::new(1),
            key_number,
            velocity: U7::new(127),
        })
    }
    pub fn tone_midi_on_msgs(tone: &Tone) -> Vec<StructuredShortMessage> {
        tone.get_notes()
            .iter()
            .flat_map(|x| Self::note_midi_on_msg(x))
            .collect()
    }
    pub fn tone_midi_off_msgs(tone: &Tone) -> Vec<StructuredShortMessage> {
        tone.get_notes()
            .iter()
            .flat_map(|x| Self::note_midi_off_msg(x))
            .collect()
    }
}
