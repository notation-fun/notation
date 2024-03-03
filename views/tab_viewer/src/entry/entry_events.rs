use edger_bevy::bevy_prelude::*;
use std::sync::Arc;

use notation_model::prelude::*;

#[derive(Clone, Debug)]
pub struct AddEntryEvent(pub Entity, pub Arc<LaneEntry>, pub BarPosition);
