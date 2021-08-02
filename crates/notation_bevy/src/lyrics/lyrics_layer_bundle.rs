use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Track;

use crate::prelude::LyricsGrid;

#[derive(Bundle)]
pub struct LyricsLayerBundle {
    grid: LyricsGrid,
}

impl LyricsLayerBundle {
    pub fn new(_track: Arc<Track>) -> Self {
        Self {
            grid: LyricsGrid {},
        }
    }
}
