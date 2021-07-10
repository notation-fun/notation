use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::sync::Arc;

use crate::prelude::{BevyConfig, LyonShapeOp};
use notation_proto::prelude::{TabBar, Units};

use super::fretted_string::{FrettedString, FrettedStringData};

pub struct FrettedGrid<const S: usize> {}

impl<const S: usize> FrettedGrid<S> {
    pub fn add_strings(
        &self,
        commands: &mut Commands,
        config: &BevyConfig,
        entity: Entity,
        tab_bar: &Arc<TabBar>,
    ) -> () {
        for string in 0..S {
            FrettedString::create(commands, entity, config, FrettedStringData::new(tab_bar, string));
        }
    }
}
