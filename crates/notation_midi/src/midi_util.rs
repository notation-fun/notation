use std::convert::TryFrom;

use helgoboss_midi::{Channel, KeyNumber, StructuredShortMessage, U7};
use notation_model::prelude::{BarPosition, CoreEntry, Entry, Fretboard, FrettedEntry, GuitarUtil, HandShape, LaneKind, ModelEntry, Note, Pick, Semitones, SliceEntry, TabBar, Tone, Units};

use crate::prelude::MidiChannel;

pub struct MidiUtil();

impl MidiUtil {
    pub fn note_midi_key_number(note: &Note) -> Option<KeyNumber> {
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
        bar: &TabBar,
        entry: &SliceEntry,
        tone: &Tone,
    ) -> Option<Vec<(BarPosition, StructuredShortMessage)>> {
        if tone.is_none() || entry.prev_is_tie() {
            return None;
        }
        let start_position = BarPosition::new(
            bar.bar_units(), bar.bar_ordinal, entry.props.in_bar_pos);
        let mut play_msgs: Vec<(BarPosition, StructuredShortMessage)> = tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_on_msg(x, channel.channel, channel.velocity))
            .map(|x| (start_position, x))
            .collect();
        let stop_position = BarPosition::new(
            bar.bar_units(), bar.bar_ordinal,
            entry.props.in_bar_pos + entry.model.tied_units());
        let mut stop_msgs: Vec<(BarPosition, StructuredShortMessage)> = tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_off_msg(x, channel.channel, channel.velocity))
            .map(|x| (stop_position, x))
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
        entry: &SliceEntry,
        core_entry: &CoreEntry
    ) -> Option<Vec<(BarPosition, StructuredShortMessage)>> {
        match core_entry {
            CoreEntry::Tone(tone, _) =>
                Self::get_tone_midi_msgs(channel, bar, entry, tone),
            _ => None,
        }
    }
    pub fn get_pick_midi_msgs<F1, F2, const S: usize>(
        as_fretted_entry: &F1,
        new_default_fretboard: &F2,
        channel: &MidiChannel,
        bar: &TabBar,
        entry: &SliceEntry,
        pick: &Pick,
    ) -> Option<Vec<(BarPosition, StructuredShortMessage)>>
    where
        F1: Fn(&ModelEntry) -> Option<&FrettedEntry<S>>,
        F2: Fn() -> Fretboard<S>,
    {
        if let Some((fretboard, shape)) =
                bar.get_fretted_shape::<F1, F2, S>(as_fretted_entry, new_default_fretboard, entry) {
            let tone = fretboard.pick_tone(&shape, pick);
            Self::get_tone_midi_msgs(channel, bar, entry, &tone)
        } else {
            None
        }
    }
    pub fn get_fretted_midi_msgs<F1, F2, const S: usize>(
        as_fretted_entry: &F1,
        new_default_fretboard: &F2,
        channel: &MidiChannel,
        bar: &TabBar,
        entry: &SliceEntry,
        fretted_entry: &FrettedEntry<S>,
    ) -> Option<Vec<(BarPosition, StructuredShortMessage)>>
    where
        F1: Fn(&ModelEntry) -> Option<&FrettedEntry<S>>,
        F2: Fn() -> Fretboard<S>,
    {
        match fretted_entry {
            FrettedEntry::Pick(pick, _) =>
                Self::get_pick_midi_msgs::<F1, F2, S>(as_fretted_entry, new_default_fretboard, channel, bar, entry, pick),
            _ => None,
        }
    }
    pub fn get_midi_msgs(
        channel: &MidiChannel,
        bar: &TabBar,
        entry: &SliceEntry,
    ) -> Option<Vec<(BarPosition, StructuredShortMessage)>> {
        match entry.proto() {
            notation_model::prelude::ProtoEntry::Core(core_entry) =>
                Self::get_core_midi_msgs(channel, bar, entry, core_entry),
            notation_model::prelude::ProtoEntry::FrettedSix(fretted_entry) => {
                Self::get_fretted_midi_msgs(
                    &ModelEntry::as_fretted_six, &GuitarUtil::new_default_fretboard,
                    channel, bar, entry, fretted_entry)
            },
            notation_model::prelude::ProtoEntry::FrettedFour(_fretted_entry) =>
                None,
            _ => None
        }
    }
}


