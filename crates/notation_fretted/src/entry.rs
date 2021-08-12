use serde::{Deserialize, Serialize};

use super::prelude::{Fretboard4, Fretboard6, HandShape4, HandShape6};
use crate::prelude::{Pick, Strum};
use notation_core::prelude::{Duration, Entry};

macro_rules! impl_entry {
    ($type:ident, $strings:literal, $hand_shape:ident, $fretboard:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
        pub enum $type {
            Pick(Pick, Duration),
            Strum(Strum, Duration),
            Shape($hand_shape, Duration),
            Fretboard($fretboard),
        }

        impl $type {
            pub fn duration(&self) -> Duration {
                match self {
                    $type::Pick(_, duration) => *duration,
                    $type::Strum(_, duration) => *duration,
                    $type::Shape(_, duration) => *duration,
                    $type::Fretboard(_) => Duration::Zero,
                }
            }
        }

        impl $type {
            /// Returns `true` if the fretted_entry is [`Fretboard`].
            pub fn is_fretboard(&self) -> bool {
                matches!(self, Self::Fretboard(..))
            }

            /// Returns `true` if the fretted_entry is [`Shape`].
            pub fn is_shape(&self) -> bool {
                matches!(self, Self::Shape(..))
            }

            /// Returns `true` if the fretted_entry is [`Pick`].
            pub fn is_pick(&self) -> bool {
                matches!(self, Self::Pick(..))
            }

            /// Returns `true` if the fretted_entry is [`Strum`].
            pub fn is_strum(&self) -> bool {
                matches!(self, Self::Strum(..))
            }
        }

        impl $type {
            pub fn as_pick(&self) -> Option<(&Pick, &Duration)> {
                if let Self::Pick(v, d) = self {
                    Some((v, d))
                } else {
                    None
                }
            }
            pub fn as_strum(&self) -> Option<(&Strum, &Duration)> {
                if let Self::Strum(v, d) = self {
                    Some((v, d))
                } else {
                    None
                }
            }
            pub fn as_shape(&self) -> Option<(&$hand_shape, &Duration)> {
                if let Self::Shape(v, d) = self {
                    Some((v, d))
                } else {
                    None
                }
            }
            pub fn as_fretboard(&self) -> Option<&$fretboard> {
                if let Self::Fretboard(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
        }

        impl Entry for $type {
            fn duration(&self) -> Duration {
                self.duration()
            }
        }

        impl From<$fretboard> for $type {
            fn from(v: $fretboard) -> Self {
                Self::Fretboard(v)
            }
        }

        impl From<($hand_shape, Duration)> for $type {
            fn from(v: ($hand_shape, Duration)) -> Self {
                Self::Shape(v.0, v.1)
            }
        }

        impl From<(Pick, Duration)> for $type {
            fn from(v: (Pick, Duration)) -> Self {
                Self::Pick(v.0, v.1)
            }
        }

        impl From<(Strum, Duration)> for $type {
            fn from(v: (Strum, Duration)) -> Self {
                Self::Strum(v.0, v.1)
            }
        }
    };
}

impl_entry!(FrettedEntry6, 6, HandShape6, Fretboard6);
impl_entry!(FrettedEntry4, 4, HandShape4, Fretboard4);
