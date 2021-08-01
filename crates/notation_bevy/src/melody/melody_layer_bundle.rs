use std::sync::Arc;
use bevy::prelude::*;

use notation_model::prelude::{Track};

use crate::prelude::{MelodyGrid, ToneMode};

#[derive(Bundle)]
pub struct MelodyLayerBundle {
    grid: MelodyGrid,
    mode: ToneMode,
}

impl MelodyLayerBundle {
    pub fn new(_track: Arc<Track>) -> Self {
        Self {
            grid: MelodyGrid {},
            mode: ToneMode::Melody,
        }
    }
}
