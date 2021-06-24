use std::sync::Arc;
use bevy::prelude::*;

use notation_proto::prelude::*;

#[derive(Clone)]
pub struct AddEntryEvent(pub Entity, pub Arc<Entry>, pub Units);
