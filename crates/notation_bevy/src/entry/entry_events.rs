use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::*;

#[derive(Clone, Debug)]
pub struct AddEntryEvent(pub Entity, pub Arc<ModelEntry>, pub BarPosition);
