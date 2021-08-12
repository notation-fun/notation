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
impl Display for Finger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Finger::Thumb => "T",
            Finger::Index => "I",
            Finger::Middle => "M",
            Finger::Ring => "R",
            Finger::Pinky => "Y",
        })
    }
}

macro_rules! impl_hand_shape {
    ($type:ident, $strings:literal) => {
        #[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
        pub struct $type {
            #[serde(with = "serde_arrays")]
            pub frets: [Option<u8>; $strings],
            #[serde(with = "serde_arrays")]
            pub fingers: [Option<Finger>; $strings],
        }
        impl Display for $type {
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

        impl $type {
            pub fn new(frets: [Option<u8>; $strings], fingers: [Option<Finger>; $strings]) -> Self {
                Self { frets, fingers }
            }
            pub fn string_fret(&self, string: u8) -> Option<u8> {
                if string == 0 || string as usize > self.frets.len() {
                    None
                } else {
                    self.frets[string as usize - 1]
                }
            }
        }

        impl From<([Option<u8>; $strings], [Option<Finger>; $strings])> for $type {
            fn from(v: ([Option<u8>; $strings], [Option<Finger>; $strings])) -> Self {
                Self::new(v.0, v.1)
            }
        }

        impl From<[Option<u8>; $strings]> for $type {
            fn from(v: [Option<u8>; $strings]) -> Self {
                Self::new(v, [None; $strings])
            }
        }
    }
}

impl_hand_shape!(HandShape6, 6);
impl_hand_shape!(HandShape4, 4);
