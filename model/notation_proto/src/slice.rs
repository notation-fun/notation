use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SliceBegin {
    Mark(String),
    Index(usize),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SliceEnd {
    Mark(String),
    Count(usize),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Slice {
    pub begin: SliceBegin,
    pub end: SliceEnd,
    pub rounds: Option<Vec<usize>>,
}
impl Default for Slice {
    fn default() -> Self {
        Self {
            begin: SliceBegin::Index(0),
            end: SliceEnd::Count(0),
            rounds: None,
        }
    }
}
impl Slice {
    pub fn new(begin: SliceBegin, end: SliceEnd, rounds: Option<Vec<usize>>) -> Self {
        Self { begin, end, rounds }
    }
    pub fn not_in_round(&self, round: usize) -> bool {
        self.rounds.is_some()
            && self
                .rounds
                .clone()
                .unwrap()
                .iter()
                .find(|&x| *x == round)
                .is_none()
    }
    pub fn in_round(&self, round: usize) -> bool {
        !self.not_in_round(round)
    }
}
impl Display for SliceBegin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for SliceEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Slice>({}-{}", self.begin, self.end)?;
        if let Some(ref rounds) = self.rounds {
            write!(f, " R:{:?}", rounds)?;
        }
        write!(f, ")")
    }
}
