use serde::{Deserialize, Serialize};

use crate::prelude::Pick;

use super::hand::HandShape;
use notation_core::prelude::{Note, Semitones, Tone};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Fretboard<const S: usize> {
    pub total_fret_num: usize,
    #[serde(with = "serde_arrays")]
    pub string_notes: [Note; S],
    pub capo: u8,
}

impl<const S: usize> Fretboard<S> {
    pub fn new(total_fret_num: usize, string_notes: [Note; S], capo: u8) -> Self {
        Self {
            total_fret_num,
            string_notes,
            capo,
        }
    }
    pub fn clone_<const S1: usize>(&self) -> Fretboard<S1> {
        if S != S1 {
            println!("Fretboard<{}> unsafe clone_: {}", S, S1);
        }
        let mut string_notes = [Note::A_0; S1];
        for i in 0..std::cmp::min(S, S1) {
            string_notes[i] = self.string_notes[i];
        }
        Fretboard::<S1> {
            total_fret_num: self.total_fret_num,
            string_notes,
            capo: self.capo,
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
    pub fn open_notes(&self) -> [Note; S] {
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
    pub fn shape_note(&self, shape: &HandShape<S>, string: u8) -> Option<Note> {
        let note = self.open_note(string);
        note?;
        match shape.string_fret(string) {
            None => None,
            Some(0) => note,
            Some(fret) => Some((Semitones::from(note.unwrap()) + Semitones(fret as i8)).into()),
        }
    }
    pub fn pick_tone(&self, shape: &HandShape<S>, pick: &Pick) -> Tone {
        let notes = match pick {
            Pick::None => vec![],
            Pick::Single(p1) => vec![self.shape_note(shape, *p1)],
            Pick::Double(p1, p2) => vec![self.shape_note(shape, *p1), self.shape_note(shape, *p2)],
            Pick::Triple(p1, p2, p3) => vec![
                self.shape_note(shape, *p1),
                self.shape_note(shape, *p2),
                self.shape_note(shape, *p3),
            ],
            Pick::Tetra(p1, p2, p3, p4) => vec![
                self.shape_note(shape, *p1),
                self.shape_note(shape, *p2),
                self.shape_note(shape, *p3),
                self.shape_note(shape, *p4),
            ],
            Pick::Penta(p1, p2, p3, p4, p5) => vec![
                self.shape_note(shape, *p1),
                self.shape_note(shape, *p2),
                self.shape_note(shape, *p3),
                self.shape_note(shape, *p4),
                self.shape_note(shape, *p5),
            ],
        };
        notes.into()
    }
}
