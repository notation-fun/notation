use std::fmt::Display;
use std::sync::{Arc};

use bevy_utils::prelude::{View};
use notation_model::prelude::Tab;

use crate::ui::layout::NotationLayout;

pub struct TabBars {
    pub tab: Arc<Tab>,
}
impl Display for TabBars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabBars>({})", self.tab.bars.len())
    }
}
impl TabBars {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self {
            tab,
        }
    }
}
impl<'a> View<NotationLayout<'a>> for TabBars {
}