use serde::{Deserialize, Serialize};

use super::hand::HandShape;
use notation_core::prelude::{Note, Semitones};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Fretboard<const S: usize> {
    pub total_fret_num: usize,
    #[serde(with = "serde_arrays")]
    pub string_notes: [Note; S],
    pub capo: u8,
}

impl<const S: usize> Fretboard<S> {
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
            return self.string_notes.clone();
        }
        self.string_notes.clone().map(|x| self.get_capo_note(x))
    }
    pub fn open_note(&self, string: u8) -> Option<Note> {
        if string as usize >= self.string_notes.len() {
            None
        } else {
            Some(self.get_capo_note(self.string_notes[string as usize]))
        }
    }
    pub fn shape_notes(&self, shape: &HandShape<S>) -> [Option<Note>; S] {
        let mut notes = self.open_notes().map(|x| Some(x));
        for (index, note) in notes.iter_mut().enumerate() {
            *note = match shape.string_fret(index as u8) {
                None => None,
                Some(0) => *note,
                Some(fret) => {
                    let open_note = (*note).unwrap();
                    Some((Semitones::from(open_note) + Semitones(fret as i8)).into())
                }
            }
        }
        notes
    }
    pub fn shape_note(&self, shape: &HandShape<S>, string: u8) -> Option<Note> {
        let note = self.open_note(string);
        if note.is_none() {
            return None;
        }
        match shape.string_fret(string) {
            None => None,
            Some(0) => note,
            Some(fret) => Some((Semitones::from(note.unwrap()) + Semitones(fret as i8)).into()),
        }
    }
}
