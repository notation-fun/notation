use helgoboss_midi::StructuredShortMessage;
use wasm_bindgen::prelude::*;

use notation_model::prelude::{Entry, PlaySpeed};
use crate::prelude::MidiMessage;

pub struct MidiSynth {}

impl MidiSynth {
    pub fn try_new() -> Option<MidiSynth> {
        Some(MidiSynth{})
    }
    pub fn send(&self, speed: &PlaySpeed, msg: &MidiMessage) -> Result<(), String> {
        match msg.midi {
            StructuredShortMessage::NoteOff {
                channel,
                key_number,
                velocity: _,
            } => Ok(()),
            StructuredShortMessage::NoteOn {
                channel,
                key_number,
                velocity,
            } => Ok(play_note(key_number.into(), speed.calc_seconds(msg.entry.tied_units()))),
            _ => Err("NOT_IMPLEMENTED".to_owned()),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn play_note(seminones: u8, seconds: f32);
}