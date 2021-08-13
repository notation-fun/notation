use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::NotationTheme;
use notation_model::prelude::TabBar;

#[derive(Debug, Default)]
pub struct LyricsGrid();

impl LyricsGrid {
    pub fn add_octave(
        &self,
        _commands: &mut Commands,
        _theme: &NotationTheme,
        _entity: Entity,
        _tab_bar: &Arc<TabBar>,
    ) {
        //TODO
    }
}
