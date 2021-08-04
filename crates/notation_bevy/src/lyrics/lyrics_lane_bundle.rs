use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Track;

use crate::prelude::LyricsGrid;

#[derive(Bundle)]
pub struct LyricsLaneBundle {
    grid: LyricsGrid,
}

impl LyricsLaneBundle {
    pub fn new(_track: Arc<Track>) -> Self {
        Self {
            grid: LyricsGrid {},
        }
    }
}
