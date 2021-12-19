use helgoboss_midi::StructuredShortMessage;
use wasm_bindgen::prelude::*;

use crate::prelude::{MidiMessage, MidiSettings, MidiState};
use notation_model::prelude::{Entry, PlaySpeed};

pub struct MidiSynth {}

impl MidiSynth {
    pub const VOLUME_FACTOR: f32 = 0.7; //The sound in browser is a bit too loud

    pub fn try_new() -> Option<MidiSynth> {
        Some(MidiSynth {})
    }
    pub fn init_channels(&self, _settings: &MidiSettings, state: &MidiState) {
        for channel in state.channels.iter() {
            if channel.track.is_some() {
                init_channel(channel.channel.into(), channel.program.into());
            }
        }
        load_instruments();
    }
    pub fn send(&self, speed: &PlaySpeed, msg: &MidiMessage, velocity: u8) -> Result<(), String> {
        match msg.midi {
            StructuredShortMessage::NoteOff {
                channel: _,
                key_number: _,
                velocity: _,
            } => {
                Ok(())
            },
            StructuredShortMessage::NoteOn {
                channel,
                key_number,
                velocity: _,
            } => {
                if velocity > 0 {
                    let mut volume = velocity as f32 / 128.0;
                    if volume > 1.0 {
                        volume = 1.0;
                    }
                    Ok(play_note(
                        channel.into(),
                        key_number.into(),
                        speed.calc_seconds(msg.duration),
                        volume * Self::VOLUME_FACTOR,
                    ))
                } else {
                    Ok(())
                }
            },
            _ => Err("NOT_IMPLEMENTED".to_owned()),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn init_channel(channel: u8, program: u8);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn load_instruments();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn play_note(channel: u8, seminones: u8, seconds: f32, volume: f32);
}
