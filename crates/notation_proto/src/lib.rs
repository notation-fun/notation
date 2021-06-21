pub use notation_core;
pub mod entry;
pub mod entry_wrap;
pub mod line;
pub mod form;

pub mod prelude {
    #[doc(hidden)]
    pub use notation_core::prelude::*;
    #[doc(hidden)]
    pub use crate::entry::{Entry};
    #[doc(hidden)]
    pub use crate::entry_wrap::{EntryWrap, ZeroEntryWrap};
    #[doc(hidden)]
    pub use crate::line::{RcLine, RcSlice, ArcLine, ArcSlice};
    #[doc(hidden)]
    pub use crate::form::{TrackKind, Track, Form};
    #[doc(hidden)]
    pub use crate::form::{RcBarLayer, RcBar, RcSection, RcTab};
    pub use crate::form::{ArcBarLayer, ArcBar, ArcSection, ArcTab};
}