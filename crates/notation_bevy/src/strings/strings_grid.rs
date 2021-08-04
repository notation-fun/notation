use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::{LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

use super::single_string::{SingleString, SingleStringData};

pub struct StringsGrid<const S: usize> {}

impl<const S: usize> StringsGrid<S> {
    pub fn add_strings(
        &self,
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        tab_bar: &Arc<TabBar>,
    ) {
        for string in 1..=S {
            SingleString::create(
                commands,
                entity,
                theme,
                SingleStringData::new(tab_bar, string as u8),
            );
        }
    }
}
