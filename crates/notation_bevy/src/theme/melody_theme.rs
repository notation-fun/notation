use serde::{Deserialize, Serialize};
use bevy::prelude::*;

use notation_model::prelude::{Note, Octave, Semitones};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MelodyTheme {
    pub syllable_mode: bool,
    pub note_height: f32,
    pub note_outline: f32,
    pub note_outline_color: Color,
    pub center_y: f32,
    pub octave_height: f32,
    pub semitone_height: f32,
}

impl Default for MelodyTheme {
    fn default() -> Self {
        Self {
            syllable_mode: true,
            note_height: 4.0,
            note_outline: 1.0,
            note_outline_color: Color::hex("AAAAAA").unwrap(),
            center_y: 64.0,
            octave_height: 12.0,
            semitone_height: 1.0,
        }
    }
}

impl MelodyTheme {
    pub fn calc_note_y(&self, note: Note) -> f32 {
        let center_octave = Octave::default(); //TODO
        let octave_diff = Semitones::from(note.octave) - Semitones::from(center_octave);
        let semitones = note
            .syllable
            .and_then(|s| {
                if self.syllable_mode {
                    Some(Semitones::from(s))
                } else {
                    None
                }
            })
            .unwrap_or(Semitones::from(note.pitch));
        self.center_y
            + self.octave_height * (octave_diff.0 as f32 / 12.0)
            + self.semitone_height * semitones.0 as f32
    }
}
