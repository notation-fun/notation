use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct HandShape<const S: usize> {
    #[serde(with = "serde_arrays")]
    pub frets: [Option<u8>; S],
    #[serde(with = "serde_arrays")]
    pub fingers: [Option<Finger>; S],
}
impl<const S: usize> Display for HandShape<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for fret in self.frets {
            match fret {
                Some(fret) => write!(f, "{}", fret)?,
                None => write!(f, "x")?,
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<const S: usize> HandShape<S> {
    pub fn new(frets: [Option<u8>; S], fingers: [Option<Finger>; S]) -> Self {
        Self { frets, fingers }
    }
    pub fn clone_<const S1: usize>(&self) -> HandShape<S1> {
        if S != S1 {
            println!("HandShape<{}> unsafe clone_: {}", S, S1);
        }
        let mut frets = [None; S1];
        let mut fingers = [None; S1];
        for i in 0..std::cmp::min(S, S1) {
            frets[i] = self.frets[i];
            fingers[i] = self.fingers[i];
        }
        HandShape::<S1> { frets, fingers }
    }
    pub fn string_fret(&self, string: u8) -> Option<u8> {
        if string as usize >= self.frets.len() {
            None
        } else {
            self.frets[string as usize]
        }
    }
}

impl<const S: usize> From<([Option<u8>; S], [Option<Finger>; S])> for HandShape<S> {
    fn from(v: ([Option<u8>; S], [Option<Finger>; S])) -> Self {
        Self::new(v.0, v.1)
    }
}

impl<const S: usize> From<[Option<u8>; S]> for HandShape<S> {
    fn from(v: [Option<u8>; S]) -> Self {
        Self::new(v, [None; S])
    }
}
