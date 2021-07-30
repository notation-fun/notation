use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::{LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

use super::fretted_string::{FrettedString, FrettedStringData};

pub struct FrettedGrid<const S: usize> {}

impl<const S: usize> FrettedGrid<S> {
    pub fn add_strings(
        &self,
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        tab_bar: &Arc<TabBar>,
    ) {
        for string in 1..=S {
            FrettedString::create(
                commands,
                entity,
                theme,
                FrettedStringData::new(tab_bar, string as u8),
            );
        }
    }
}
