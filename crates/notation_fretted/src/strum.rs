use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum StrumDirection {
    Down,
    Up,
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