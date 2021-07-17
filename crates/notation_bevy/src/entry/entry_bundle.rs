use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::*;

use crate::prelude::EntryState;

#[derive(Bundle)]
pub struct EntryBundle {
    pub entry: Arc<ProtoEntry>,
    pub duration: Duration,
    pub position: BarPosition,
    pub state: EntryState,
}

impl From<(Arc<ProtoEntry>, BarPosition)> for EntryBundle {
    fn from(v: (Arc<ProtoEntry>, BarPosition)) -> Self {
        let duration = v.0.duration();
        EntryBundle {
            entry: v.0,
            duration,
            position: v.1,
            state: EntryState::default(),
        }
    }
}
