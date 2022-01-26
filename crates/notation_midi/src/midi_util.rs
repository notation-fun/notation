use std::convert::TryFrom;

use helgoboss_midi::{Channel, KeyNumber, StructuredShortMessage, U7};
use notation_model::prelude::{
    CoreEntry, Entry, FrettedEntry4, FrettedEntry6, LaneEntry, Note, Pick, Semitones, TabBar, Tone,
};

use crate::prelude::MidiChannel;

pub struct MidiUtil();

impl MidiUtil {
    pub fn note_midi_key_number(note: &Note) -> Option<KeyNumber> {
        #[cfg(not(target_arch = "wasm32"))]
        let midi_note = Semitones::from(*note).0 + 12 - 1; //Not sure why got a higher pitch when playing, temp fix for get it right in video
        #[cfg(target_arch = "wasm32")]
        let midi_note = Semitones::from(*note).0 + 12;
        KeyNumber::try_from(midi_note as u8).ok()
    }
    pub fn note_midi_on_msg(
        note: &Note,
        channel: Channel,
        velocity: U7,
    ) -> Option<StructuredShortMessage> {
        Self::note_midi_key_number(note).map(|key_number| -> StructuredShortMessage {
            StructuredShortMessage::NoteOn {
                channel,
                key_number,
                velocity,
            }
        })
    }
    pub fn note_midi_off_msg(
        note: &Note,
        channel: Channel,
        velocity: U7,
    ) -> Option<StructuredShortMessage> {
        Self::note_midi_key_number(note).map(|key_number| StructuredShortMessage::NoteOff {
            channel,
            key_number,
            velocity,
        })
    }
    pub fn get_tone_midi_msgs(
        channel: &MidiChannel,
        _bar: &TabBar,
        entry: &LaneEntry,
        tone: &Tone,
    ) -> Option<Vec<(bool, StructuredShortMessage)>> {
        if tone.is_none() || entry.prev_is_tie() {
            return None;
        }
        let mut play_msgs: Vec<(bool, StructuredShortMessage)> = tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_on_msg(x, channel.channel, channel.velocity))
            .map(|x| (false, x))
            .collect();
        let mut stop_msgs: Vec<(bool, StructuredShortMessage)> = tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_off_msg(x, channel.channel, channel.velocity))
            .map(|x| (true, x))
            .collect();
        play_msgs.append(&mut stop_msgs);
        if play_msgs.len() > 0 {
            Some(play_msgs)
        } else {
            None
        }
    }
    pub fn get_core_midi_msgs(
        channel: &MidiChannel,
        bar: &TabBar,
        entry: &LaneEntry,
        core_entry: &CoreEntry,
    ) -> Option<Vec<(bool, StructuredShortMessage)>> {
        match core_entry {
            CoreEntry::Tone(tone, _) => Self::get_tone_midi_msgs(channel, bar, entry, tone),
            _ => None,
        }
    }
    pub fn get_midi_msgs(
        channel: &MidiChannel,
        bar: &TabBar,
        entry: &LaneEntry,
    ) -> Option<Vec<(bool, StructuredShortMessage)>> {
        match entry.proto() {
            notation_model::prelude::ProtoEntry::Core(core_entry) => {
                Self::get_core_midi_msgs(channel, bar, entry, core_entry)
            }
            notation_model::prelude::ProtoEntry::Fretted6(fretted_entry) => {
                Self::get_fretted_midi_msgs6(channel, bar, entry, fretted_entry)
            }
            notation_model::prelude::ProtoEntry::Fretted4(fretted_entry) => {
                Self::get_fretted_midi_msgs4(channel, bar, entry, fretted_entry)
            }
            _ => None,
        }
    }
}

macro_rules! impl_get_pick_midi_msgs {
    ($name:ident, $get_fretted_shape:ident) => {
        impl MidiUtil {
            pub fn $name(
                channel: &MidiChannel,
                bar: &TabBar,
                entry: &LaneEntry,
                pick: &Pick,
            ) -> Option<Vec<(bool, StructuredShortMessage)>> {
                if let Some((fretboard, shape)) = bar.$get_fretted_shape(entry) {
                    let meta = bar.tab_meta();
                    let tone = fretboard.pick_tone(&meta.scale, &meta.key, &shape, pick);
                    Self::get_tone_midi_msgs(channel, bar, entry, &tone)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! impl_get_fretted_midi_msgs {
    ($name:ident, $get_pick_midi_msgs:ident, $fretted_entry:ident) => {
        impl MidiUtil {
            pub fn $name(
                channel: &MidiChannel,
                bar: &TabBar,
                entry: &LaneEntry,
                fretted_entry: &$fretted_entry,
            ) -> Option<Vec<(bool, StructuredShortMessage)>> {
                match fretted_entry {
                    $fretted_entry::Pick(pick, _) => {
                        Self::$get_pick_midi_msgs(channel, bar, entry, pick)
                    }
                    _ => None,
                }
            }
        }
    };
}

impl_get_pick_midi_msgs!(get_pick_midi_msgs6, get_fretted_shape6);
impl_get_pick_midi_msgs!(get_pick_midi_msgs4, get_fretted_shape4);

impl_get_fretted_midi_msgs!(get_fretted_midi_msgs6, get_pick_midi_msgs6, FrettedEntry6);
impl_get_fretted_midi_msgs!(get_fretted_midi_msgs4, get_pick_midi_msgs4, FrettedEntry4);
