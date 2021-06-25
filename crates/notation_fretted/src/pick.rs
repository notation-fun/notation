use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Pick {
    Single (u8),
    Double (u8, u8),
    Triple (u8, u8, u8),
    Quadra (u8, u8, u8, u8),
    Penta (u8, u8, u8, u8, u8),
}