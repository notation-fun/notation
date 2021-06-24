use std::sync::Arc;
use bevy::prelude::*;

use notation_core::prelude::Duration;
use notation_proto::prelude::*;

#[derive(Bundle)]
pub struct EntryBundle {
    pub entry: Arc<Entry>,
    pub duration: Duration,
    pub position: Units,
}

impl From<(Arc<Entry>, Units)> for EntryBundle {
    fn from(v: (Arc<Entry>, Units)) -> Self {
        EntryBundle {
            entry: v.0.clone(),
            duration: v.0.duration(),
            position: v.1,
        }
    }
}