pub mod asset;
pub mod bundle;
pub mod layout;
pub mod plugin;
pub mod shape;
pub mod util;
pub mod view;

//#[cfg(feature = "dev")]
pub mod dev;

#[cfg(feature = "egui")]
pub use bevy_egui;

#[cfg(feature = "egui")]
pub mod easy_mark;

#[cfg(feature = "egui")]
pub mod egui;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::asset::markdown_asset::MarkDownAsset;
    #[doc(hidden)]
    pub use crate::bundle::single_bundle::{SingleBundle, SingleData};
    #[doc(hidden)]
    pub use crate::bundle::view_bundle::ViewBundle;
    #[doc(hidden)]
    pub use crate::layout::anchor::{LayoutAnchor, LayoutHAnchor, LayoutVAnchor};
    #[doc(hidden)]
    pub use crate::layout::data::{LayoutConstraint, LayoutData, LayoutSize};
    #[doc(hidden)]
    pub use crate::layout::dock::{DockPanel, DockSide, DockView};
    #[doc(hidden)]
    pub use crate::layout::grid::{GridCell, GridCellData, GridCellSize, GridData, GridView};
    #[doc(hidden)]
    pub use crate::layout::vbox::{VBoxCell, VBoxCellData, VBoxView};
    #[doc(hidden)]
    pub use crate::layout::view::{
        DoLayoutEvent, LayoutChangedQuery, LayoutChangedWithChildrenQuery, LayoutEnv, LayoutQuery,
        View, ViewAddedQuery, ViewEntity, ViewQuery, ViewRootAddedQuery, ViewRootQuery,
    };
    #[doc(hidden)]
    pub use crate::plugin::UtilsPlugin;
    #[doc(hidden)]
    pub use crate::shape::circle::{FillCircle, OutlineCircle, StrokeCircle};
    #[doc(hidden)]
    pub use crate::shape::line::StrokeLine;
    #[doc(hidden)]
    pub use crate::shape::path::{FillPath, StrokeCirclePath, StrokePath, StrokeRectanglePath};
    #[doc(hidden)]
    pub use crate::shape::rectangle::{FillRectangle, OutlineRectangle, StrokeRectangle};
    #[doc(hidden)]
    pub use crate::shape::shape::{ShapeOp, SingleShape};
    #[doc(hidden)]
    pub use crate::util::BevyUtil;
    #[doc(hidden)]
    pub use crate::view::color_background::ColorBackground;
    #[cfg(feature = "egui")]
    #[doc(hidden)]
    pub use crate::egui::*;
}

/// Create a [`Hyperlink`](crate::Hyperlink) to this egui source code file on github.
#[doc(hidden)]
#[macro_export]
macro_rules! __egui_github_link_file {
    () => {
        crate::__egui_github_link_file!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file!(
            "https://github.com/emilk/egui/blob/master/",
            egui::RichText::new($label).small()
        )
    };
}

/// Create a [`Hyperlink`](crate::Hyperlink) to this egui source code file and line on github.
#[doc(hidden)]
#[macro_export]
macro_rules! __egui_github_link_file_line {
    () => {
        crate::__egui_github_link_file_line!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file_line!(
            "https://github.com/emilk/egui/blob/master/",
            egui::RichText::new($label).small()
        )
    };
}