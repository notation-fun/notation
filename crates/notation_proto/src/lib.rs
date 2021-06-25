pub use notation_core;
pub mod entry;
pub mod line;
pub mod form;

pub mod prelude {
    #[doc(hidden)]
    pub use notation_core::prelude::*;
    #[doc(hidden)]
    pub use crate::entry::{ProtoEntry};
    #[doc(hidden)]
    pub use crate::line::{RcLine, RcSlice};
    #[doc(hidden)]
    pub use crate::line::{ArcLine, ArcSlice};
    #[doc(hidden)]
    pub use crate::form::{TrackKind, Track, Form};
    #[doc(hidden)]
    pub use crate::form::{RcBarLayer, RcBar, RcSection, RcTab};
    #[doc(hidden)]
    pub use crate::form::{ArcBarLayer, ArcBar, ArcSection, ArcTab};
}