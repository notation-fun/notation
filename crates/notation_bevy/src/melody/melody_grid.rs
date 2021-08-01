use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::{LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

pub struct MelodyGrid {}

impl MelodyGrid {
    pub fn add_octave(
        &self,
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        tab_bar: &Arc<TabBar>,
    ) {
        //TODO
    }
}
