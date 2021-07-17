#![feature(arc_new_cyclic)]

pub use {notation_core, notation_fretted, notation_guitar};

pub mod bar;
pub mod entry;
pub mod line;
pub mod position;
pub mod section;
pub mod tab;
pub mod track;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::bar::{Bar, BarLayer};
    #[doc(hidden)]
    pub use crate::entry::ProtoEntry;
    #[doc(hidden)]
    pub use crate::line::{Line, Slice};
    #[doc(hidden)]
    pub use crate::position::{BarPosition, Position, TabPosition};
    #[doc(hidden)]
    pub use crate::section::{Form, Section, SectionKind};
    #[doc(hidden)]
    pub use crate::tab::{Tab, TabMeta};
    #[doc(hidden)]
    pub use crate::track::{Track, TrackKind};
    #[doc(hidden)]
    pub use notation_core::prelude::*;
    #[doc(hidden)]
    pub use notation_fretted::prelude::*;
    #[doc(hidden)]
    pub use notation_guitar::prelude::*;
}
