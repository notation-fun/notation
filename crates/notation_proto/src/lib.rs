pub use uuid;
pub use {notation_core, notation_fretted, notation_guitar};

pub mod bar;
pub mod lyric_entry;
pub mod position;
pub mod proto_entry;
pub mod section;
pub mod slice;
pub mod tab;
pub mod track;
pub mod prelude {
    #[doc(hidden)]
    pub use uuid::Uuid;
    #[doc(hidden)]
    pub use crate::bar::{Bar, BarLayer};
    #[doc(hidden)]
    pub use crate::lyric_entry::{LyricEntry, LyricWord};
    #[doc(hidden)]
    pub use crate::position::{BarPosition, Position, TabPosition};
    #[doc(hidden)]
    pub use crate::proto_entry::ProtoEntry;
    #[doc(hidden)]
    pub use crate::section::{Form, Section, SectionKind};
    #[doc(hidden)]
    pub use crate::slice::{Slice, SliceBegin, SliceEnd};
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
