use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Duration, Entry};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LyricWord {
    pub text: String,
}
impl Display for LyricWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
impl From<String> for LyricWord {
    fn from(v: String) -> Self {
        LyricWord { text: v }
    }
}

impl From<&str> for LyricWord {
    fn from(v: &str) -> Self {
        LyricWord::from(String::from(v))
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum LyricEntry {
    Word(LyricWord, Duration),
}
impl Display for LyricEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LyricEntry::Word(word, duration) => write!(f, "Word({}, {})", word, duration),
        }
    }
}
impl LyricEntry {
    pub fn duration(&self) -> Duration {
        match self {
            Self::Word(_, duration) => *duration,
        }
    }
}

impl Entry for LyricEntry {
    fn duration(&self) -> Duration {
        self.duration()
    }
}

impl From<(LyricWord, Duration)> for LyricEntry {
    fn from(v: (LyricWord, Duration)) -> Self {
        LyricEntry::Word(v.0, v.1)
    }
}

impl From<(String, Duration)> for LyricEntry {
    fn from(v: (String, Duration)) -> Self {
        LyricEntry::Word(LyricWord::from(v.0), v.1)
    }
}

impl From<(&str, Duration)> for LyricEntry {
    fn from(v: (&str, Duration)) -> Self {
        LyricEntry::Word(LyricWord::from(v.0), v.1)
    }
}
