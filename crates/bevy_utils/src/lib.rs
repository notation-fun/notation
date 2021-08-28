pub mod bundle;
pub mod layout;
pub mod lyon;
pub mod util;
pub mod view;

//#[cfg(feature = "dev")]
pub mod dev;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::bundle::single_bundle::{SingleArcBundle, SingleBundle};
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
    pub use crate::lyon::lyon_shape::{LyonShape, LyonShapeOp, NoThemeLyonShape};
    #[doc(hidden)]
    pub use crate::util::BevyUtil;
    #[doc(hidden)]
    pub use crate::view::color_background::ColorBackground;
}
