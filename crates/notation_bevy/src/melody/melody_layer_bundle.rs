use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::Track;

use crate::prelude::{MemoryGrid, ToneMode};

#[derive(Bundle)]
pub struct MelodyLayerBundle {
    grid: MemoryGrid,
    mode: ToneMode,
}

impl MelodyLayerBundle {
    pub fn new(_track: Arc<Track>) -> Self {
        Self {
            grid: MemoryGrid {},
            mode: ToneMode::Melody,
        }
    }
}
