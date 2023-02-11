use bevy::prelude::*;
use notation_bevy_utils::prelude::SingleData;
use std::sync::Arc;

use notation_model::prelude::*;

use crate::prelude::EntryPlaying;

#[derive(Bundle)]
pub struct EntryBundle {
    pub name: Name,
    pub entry: SingleData<LaneEntry>,
    pub playing: EntryPlaying,
}

impl From<Arc<LaneEntry>> for EntryBundle {
    fn from(v: Arc<LaneEntry>) -> Self {
        let playing = EntryPlaying::new(&v, PlayingState::Idle);
        EntryBundle {
            name: v.to_string().as_str().into(),
            entry: SingleData::<LaneEntry>(v.as_ref().clone()),
            playing,
        }
    }
}
