use bevy::prelude::*;
use std::sync::Arc;

use notation_core::prelude::Duration;
use notation_proto::prelude::*;

#[derive(Bundle)]
pub struct EntryBundle {
    pub entry: Arc<ProtoEntry>,
    pub duration: Duration,
    pub position: Units,
}

impl From<(Arc<ProtoEntry>, Units)> for EntryBundle {
    fn from(v: (Arc<ProtoEntry>, Units)) -> Self {
        let duration = v.0.duration().clone();
        EntryBundle {
            entry: v.0,
            duration: duration,
            position: v.1,
        }
    }
}
