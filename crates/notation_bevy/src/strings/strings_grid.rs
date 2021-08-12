use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::{LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

use super::single_string::{SingleString, SingleStringData};

macro_rules! impl_strings_grid {
    ($type:ident, $strings: literal) => {
        pub struct $type {}

        impl $type {
            pub fn add_strings(
                &self,
                commands: &mut Commands,
                theme: &NotationTheme,
                entity: Entity,
                tab_bar: &Arc<TabBar>,
            ) {
                for string in 1..=$strings {
                    SingleString::create(
                        commands,
                        entity,
                        theme,
                        SingleStringData::new(tab_bar, string as u8),
                    );
                }
            }
        }
    };
}

impl_strings_grid!(StringsGrid6, 6);
impl_strings_grid!(StringsGrid4, 4);
