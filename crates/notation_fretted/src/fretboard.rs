use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::pick::PickNote;
use crate::prelude::Pick;

use super::prelude::{HandShape4, HandShape6};
use notation_core::prelude::{Note, Semitones, Tone};

macro_rules! impl_fretboard {
    ($type:ident, $strings:literal, $hand_shape:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
        pub struct $type {
            pub total_fret_num: usize,
            #[serde(with = "serde_arrays")]
            pub string_notes: [Note; $strings],
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
            pub fn new(total_fret_num: usize, string_notes: [Note; $strings], capo: u8) -> Self {
                Self {
                    total_fret_num,
                    string_notes,
                    capo,
                }
            }
            pub fn with_capo(&self, capo: u8) -> Self {
                Self { capo, ..*self }
            }
            pub fn fretted_note(&self, string: u8, fret: u8) -> Option<Note> {
                if fret as usize >= self.fret_num() {
                    None
                } else if fret == 0 {
                    self.open_note(string)
                } else {
                    self.open_note(string)
                        .map(|note| (Semitones::from(note) + Semitones(fret as i8)).into())
                }
            }
            pub fn string_num(&self) -> usize {
                self.string_notes.len()
            }
            pub fn fret_num(&self) -> usize {
                self.total_fret_num - self.capo as usize
            }
            fn get_capo_note(&self, note: Note) -> Note {
                (Semitones::from(note) + Semitones(self.capo as i8)).into()
            }
            pub fn open_notes(&self) -> [Note; $strings] {
                if self.capo == 0 {
                    return self.string_notes;
                }
                self.string_notes.map(|x| self.get_capo_note(x))
            }
            /// string is 1-based.
            pub fn open_note(&self, string: u8) -> Option<Note> {
                if string == 0 || string as usize > self.string_notes.len() {
                    None
                } else {
                    Some(self.get_capo_note(self.string_notes[(string - 1) as usize]))
                }
            }
            pub fn shape_note(&self, shape: &$hand_shape, string: u8) -> Option<Note> {
                shape
                    .string_fret(string)
                    .and_then(|fret| self.fretted_note(string, fret))
            }
            pub fn shape_fret_note(&self, shape: &$hand_shape, string: u8) -> Option<(u8, Note)> {
                shape
                    .string_fret(string)
                    .and_then(|fret| self.fretted_note(string, fret).map(|n| (fret, n)))
            }
            pub fn shape_pick_note(
                &self,
                shape: &$hand_shape,
                pick_note: PickNote,
            ) -> Option<Note> {
                match pick_note.fret {
                    Some(fret) => self.fretted_note(pick_note.string, fret),
                    None => self.shape_note(shape, pick_note.string),
                }
            }
            pub fn shape_pick_fret_note(
                &self,
                shape: &$hand_shape,
                pick_note: PickNote,
            ) -> Option<(u8, Note)> {
                match pick_note.fret {
                    Some(fret) => self
                        .fretted_note(pick_note.string, fret)
                        .map(|note| (fret, note)),
                    None => self.shape_fret_note(shape, pick_note.string),
                }
            }
            pub fn pick_tone(&self, shape: &$hand_shape, pick: &Pick) -> Tone {
                let notes = match pick {
                    Pick::None => vec![],
                    Pick::Single(p1) => vec![self.shape_pick_note(shape, *p1)],
                    Pick::Double(p1, p2) => vec![
                        self.shape_pick_note(shape, *p1),
                        self.shape_pick_note(shape, *p2),
                    ],
                    Pick::Triple(p1, p2, p3) => vec![
                        self.shape_pick_note(shape, *p1),
                        self.shape_pick_note(shape, *p2),
                        self.shape_pick_note(shape, *p3),
                    ],
                    Pick::Tetra(p1, p2, p3, p4) => vec![
                        self.shape_pick_note(shape, *p1),
                        self.shape_pick_note(shape, *p2),
                        self.shape_pick_note(shape, *p3),
                        self.shape_pick_note(shape, *p4),
                    ],
                    Pick::Penta(p1, p2, p3, p4, p5) => vec![
                        self.shape_pick_note(shape, *p1),
                        self.shape_pick_note(shape, *p2),
                        self.shape_pick_note(shape, *p3),
                        self.shape_pick_note(shape, *p4),
                        self.shape_pick_note(shape, *p5),
                    ],
                };
                notes.into()
            }
        }
    };
}

impl_fretboard!(Fretboard6, 6, HandShape6);
impl_fretboard!(Fretboard4, 4, HandShape4);
