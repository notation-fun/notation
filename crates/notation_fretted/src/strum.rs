use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum StrumDirection {
    Down,
    Up,
}
impl Display for StrumDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StrumDirection::Down => "D",
            StrumDirection::Up => "U",
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum StrumStrings {
    All,
    Between(u8, u8),
}
impl Display for StrumStrings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrumStrings::All => write!(f, "All"),
            StrumStrings::Between(x, y) => write!(f, "{}-{}", x, y),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Strum {
    pub strings: StrumStrings,
    pub direction: Option<StrumDirection>,
}
impl Display for Strum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Some(x) => write!(f, "{} {}", x, self.strings),
            None => write!(f, "{}", self.strings),
        }
    }
}
