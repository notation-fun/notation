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

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Strum {
    pub string: StrumStrings,
    pub direction: Option<StrumDirection>,
}
