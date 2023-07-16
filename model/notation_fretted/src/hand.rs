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
            pub barre: Option<u8>,
            #[serde(with = "serde_arrays")]
            pub frets: [Option<u8>; $strings],
            #[serde(with = "serde_arrays")]
            pub fingers: [Option<Finger>; $strings],
        }
        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[")?;
                for index in 0..self.frets.len() {
                    let fret = self.frets[self.frets.len() - 1 - index];
                    match fret {
                        Some(fret) => write!(f, "{}", fret)?,
                        None => write!(f, "x")?,
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
        }
        impl Default for $type {
            fn default() -> Self {
                let frets = [Some(0); $strings];
                let fingers = [None; $strings];
                Self::new(frets, fingers)
            }
        }
        impl $type {
            pub fn new_barre(
                barre: u8,
                frets: [Option<u8>; $strings],
                fingers: [Option<Finger>; $strings],
            ) -> Self {
                if barre == 0 {
                    Self {
                        barre: None,
                        frets,
                        fingers,
                    }
                } else {
                    Self {
                        barre: Some(barre),
                        frets,
                        fingers,
                    }
                }
            }
            pub fn new(frets: [Option<u8>; $strings], fingers: [Option<Finger>; $strings]) -> Self {
                Self {
                    barre: None,
                    frets,
                    fingers,
                }
            }
            pub fn barre(&self) -> u8 {
                self.barre.unwrap_or(0)
            }
            pub fn string_fret(&self, string: u8) -> Option<u8> {
                if string == 0 || string as usize > self.frets.len() {
                    None
                } else {
                    self.frets[string as usize - 1]
                }
            }
            pub fn string_fret_with_barre(&self, string: u8) -> Option<u8> {
                self.string_fret(string).map(|x| x + self.barre())
            }
            pub fn max_fret(&self) -> u8 {
                let mut max = 0;
                for index in 0..self.frets.len() {
                    if let Some(fret) = self.frets[index] {
                        if fret > max {
                            max = fret
                        }
                    }
                }
                max
            }
            pub fn max_fret_with_barre(&self) -> u8 {
                self.max_fret() + self.barre()
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

        impl From<(u8, [Option<u8>; $strings], [Option<Finger>; $strings])> for $type {
            fn from(v: (u8, [Option<u8>; $strings], [Option<Finger>; $strings])) -> Self {
                Self::new_barre(v.0, v.1, v.2)
            }
        }

        impl From<(u8, [Option<u8>; $strings])> for $type {
            fn from(v: (u8, [Option<u8>; $strings])) -> Self {
                Self::new_barre(v.0, v.1, [None; $strings])
            }
        }
    };
}

impl_hand_shape!(HandShape6, 6);
impl_hand_shape!(HandShape4, 4);
