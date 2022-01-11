#![feature(proc_macro_diagnostic)]

#[macro_use]
extern crate lazy_static;

pub use {proc_macro2, quote, syn};

pub mod context;
pub mod core;
pub mod fretted;
pub mod get_tab;
pub mod helper;
pub mod proto;
pub mod util;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::get_tab::GetTabDsl;
    #[doc(hidden)]
    pub use crate::helper::*;
    #[doc(hidden)]
    pub use crate::proto::bar::BarDsl;
    #[doc(hidden)]
    pub use crate::proto::entry::EntryDsl;
    #[doc(hidden)]
    pub use crate::proto::form::FormDsl;
    #[doc(hidden)]
    pub use crate::proto::section::SectionDsl;
    #[doc(hidden)]
    pub use crate::proto::slice::SliceDsl;
    #[doc(hidden)]
    pub use crate::proto::tab::TabDsl;
    #[doc(hidden)]
    pub use crate::proto::track::TrackDsl;
    #[doc(hidden)]
    pub use crate::context::Context;
    #[doc(hidden)]
    pub use notation_proto::prelude::*;
}
