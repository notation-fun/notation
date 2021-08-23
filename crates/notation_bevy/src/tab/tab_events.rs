use notation_model::prelude::*;
use std::sync::Arc;

use crate::bar::bar_view::BarView;
use crate::prelude::TabBars;
use crate::ui::layout::NotationLayout;
use bevy_utils::prelude::DoLayoutEvent;

use super::tab_chords::TabChords;
use super::tab_content::TabContent;

#[derive(Debug)]
pub struct AddTabEvent(pub Arc<Tab>);

pub type TabContentDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabContent>;
pub type TabChordsDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabChords>;
pub type TabBarsDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabBars>;
pub type BarViewDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, BarView>;
