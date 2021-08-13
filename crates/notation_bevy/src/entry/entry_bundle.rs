use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::*;

use crate::prelude::EntryState;

#[derive(Bundle)]
pub struct EntryBundle {
    pub name: Name,
    pub entry: Arc<LaneEntry>,
    pub duration: Duration,
    pub tied_units: Units,
    pub position: BarPosition,
    pub state: EntryState,
}

impl From<(Arc<LaneEntry>, BarPosition)> for EntryBundle {
    fn from(v: (Arc<LaneEntry>, BarPosition)) -> Self {
        let duration = v.0.duration();
        let tied_units = v.0.tied_units();
        EntryBundle {
            name: v.0.to_string().as_str().into(),
            entry: v.0,
            duration,
            tied_units,
            position: v.1,
            state: EntryState::default(),
        }
    }
}
