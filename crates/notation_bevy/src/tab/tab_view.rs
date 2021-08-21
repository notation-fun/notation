use std::fmt::Display;
use std::sync::{Arc};

use bevy_utils::prelude::{DockView, View};
use notation_model::prelude::Tab;

use crate::mini::mini_map::MiniMap;
use crate::prelude::TabBars;
use crate::ui::layout::NotationLayout;

pub struct TabView {
    pub tab: Arc<Tab>,
}
impl Display for TabView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabView>({})", self.tab.bars.len())
    }
}
impl TabView {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self {
            tab,
        }
    }
}
impl<'a> View<NotationLayout<'a>> for TabView {
}
impl<'a> DockView<NotationLayout<'a>, MiniMap, TabBars> for TabView {
}
