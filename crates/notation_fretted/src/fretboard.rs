use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::pick::PickNote;
use crate::prelude::Pick;

use super::prelude::{HandShape4, HandShape6};
use notation_core::prelude::{Note, Semitones, Tone, Scale, Key};

macro_rules! impl_fretboard {
    ($type:ident, $strings:literal, $hand_shape:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
        pub struct $type {
            pub total_fret_num: usize,
            #[serde(with = "serde_arrays")]
            pub string_notes: [Semitones; $strings],
            pub capo: u8,
        }
        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "F: {}, C: {}, {:?})",
                    self.total_fret_num, self.capo, self.string_notes
                )
            }
        }

        impl $type {
            pub fn new(total_fret_num: usize, string_notes: [Semitones; $strings], capo: u8) -> Self {
                Self {
                    total_fret_num,
                    string_notes,
                    capo,
                }
            }
            pub fn with_capo(&self, capo: u8) -> Self {
                Self { capo, ..*self }
            }
            pub fn fretted_note(&self, scale: &Scale, key: &Key, string: u8, fret: u8) -> Option<Note> {
                if fret as usize >= self.fret_num() {
                    None
                } else if fret == 0 {
                    self.open_note(scale, key, string)
                } else {
                    if string == 0 || string as usize > self.string_notes.len() {
                        None
                    } else {
                        let semitones = self.string_notes[(string - 1) as usize];
                        Some(self.get_capo_note(scale, key, semitones + Semitones(fret as i8)))
                    }
                }
            }
            pub fn string_num(&self) -> usize {
                self.string_notes.len()
            }
            pub fn fret_num(&self) -> usize {
                self.total_fret_num - self.capo as usize
            }
            fn get_capo_note(&self, scale: &Scale, key: &Key, note: Semitones) -> Note {
                scale.calc_note_from_semitones(key, note + Semitones(self.capo as i8))
            }
            pub fn open_notes(&self, scale: &Scale, key: &Key) -> [Note; $strings] {
                self.string_notes.map(|x| self.get_capo_note(scale, key, x))
            }
            /// string is 1-based.
            pub fn open_note(&self, scale: &Scale, key: &Key, string: u8) -> Option<Note> {
                if string == 0 || string as usize > self.string_notes.len() {
                    None
                } else {
                    Some(self.get_capo_note(scale, key, self.string_notes[(string - 1) as usize]))
                }
            }
            pub fn shape_note(&self, scale: &Scale, key: &Key, shape: &$hand_shape, string: u8) -> Option<Note> {
                shape
                    .string_fret_with_barre(string)
                    .and_then(|fret| self.fretted_note(scale, key, string, fret))
            }
            pub fn shape_fret_note(&self, scale: &Scale, key: &Key, shape: &$hand_shape, string: u8) -> Option<(u8, Note)> {
                shape
                    .string_fret_with_barre(string)
                    .and_then(|fret| self.fretted_note(scale, key, string, fret).map(|n| (fret, n)))
            }
            pub fn shape_pick_note(
                &self, scale: &Scale, key: &Key,
                shape: &$hand_shape,
                pick_note: PickNote,
            ) -> Option<Note> {
                match pick_note.fret {
                    Some(fret) => self.fretted_note(scale, key, pick_note.string, fret),
                    None => self.shape_note(scale, key, shape, pick_note.string),
                }
            }
            pub fn shape_pick_fret_note(
                &self, scale: &Scale, key: &Key,
                shape: &$hand_shape,
                pick_note: PickNote,
            ) -> Option<(u8, Note)> {
                match pick_note.fret {
                    Some(fret) => self
                        .fretted_note(scale, key, pick_note.string, fret)
                        .map(|note| (fret, note)),
                    None => self.shape_fret_note(scale, key, shape, pick_note.string),
                }
            }
            pub fn pick_tone(&self, scale: &Scale, key: &Key, shape: &$hand_shape, pick: &Pick) -> Tone {
                let notes: Vec<Option<Note>> = pick
                    .get_notes()
                    .into_iter()
                    .map(|x| self.shape_pick_note(scale, key, shape, x))
                    .collect();
                notes.into()
            }
        }
    };
}

impl_fretboard!(Fretboard6, 6, HandShape6);
impl_fretboard!(Fretboard4, 4, HandShape4);
