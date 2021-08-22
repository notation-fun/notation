use notation_model::prelude::*;
use std::sync::Arc;

use crate::ui::layout::NotationLayout;
use bevy_utils::prelude::DoLayoutEvent;

use super::{tab_chords::TabChords, tab_content::TabContent};

#[derive(Debug)]
pub struct AddTabEvent(pub Arc<Tab>);

pub type TabContentDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabContent>;
pub type TabChordsDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabChords>;
