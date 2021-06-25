#![feature(array_map)]

pub mod fretted;
pub mod hand;
pub mod pick;
pub mod strum;
pub mod entry;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::fretted::{Fretted, Fretboard, WithCapo};
    #[doc(hidden)]
    pub use crate::hand::{HandShape};
    #[doc(hidden)]
    pub use crate::pick::{Pick};
    #[doc(hidden)]
    pub use crate::strum::{Strum};
    #[doc(hidden)]
    pub use crate::entry::{FrettedEntry};
}