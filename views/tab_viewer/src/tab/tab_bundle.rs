use edger_bevy::bevy_prelude::*;
use edger_bevy::prelude::LayoutData;
use std::sync::Arc;

use notation_model::prelude::Tab;

use super::tab_state::TabState;
use super::tab_view::TabView;

#[derive(Bundle)]
pub struct TabBundle {
    pub name: Name,
    //pub tab: Arc<Tab>,
    pub state: TabState,
    pub view: TabView,
    pub layout: LayoutData,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl TabBundle {
    pub fn new(tab: Arc<Tab>) -> Self {
        let name = tab.to_string().as_str().into();
        let state = TabState::new(&tab);
        let view =TabView::new(tab.clone());
        Self {
            name,
            //tab,
            state,
            view,
            layout: LayoutData::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
