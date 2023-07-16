use edger_bevy_app::bevy_prelude::*;

use crate::prelude::NotationTheme;
use edger_bevy_app::prelude::ShapeOp;
use notation_model::prelude::BarLane;

use super::single_string::{SingleStringData, SingleStringValue};

macro_rules! impl_strings_grid {
    ($type:ident, $strings: literal) => {
        #[derive(Debug, Default, Component)]
        pub struct $type();

        impl $type {
            pub fn add_strings(
                &self,
                commands: &mut Commands,
                theme: &NotationTheme,
                entity: Entity,
                lane: &BarLane,
            ) {
                for string in 1..=$strings {
                    let data = SingleStringData::new(lane, SingleStringValue {
                        string: string as u8,
                        bar_size: 0.0,
                    });
                    data.create(commands, theme, entity);
                }
            }
        }
    };
}

impl_strings_grid!(StringsGrid6, 6);
impl_strings_grid!(StringsGrid4, 4);
