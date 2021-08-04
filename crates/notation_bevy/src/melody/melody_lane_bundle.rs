use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Track;

use crate::prelude::{MemoryGrid, ToneMode};

#[derive(Bundle)]
pub struct MelodyLaneBundle {
    grid: MemoryGrid,
    mode: ToneMode,
}

impl MelodyLaneBundle {
    pub fn new(_track: Arc<Track>) -> Self {
        Self {
            grid: MemoryGrid {},
            mode: ToneMode::Melody,
        }
    }
}
