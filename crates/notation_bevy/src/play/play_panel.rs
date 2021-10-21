use std::fmt::Display;

use bevy::prelude::*;
use bevy_utils::prelude::{
    BevyUtil, ColorBackground, View, ViewBundle,
};
use notation_model::prelude::{Syllable};

use crate::{prelude::{NotationAssets, NotationTheme}, ui::layout::NotationLayout};

pub struct PlayPanel {
    pub playing: bool,
    pub should_loop: bool,
}

impl Display for PlayPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PlayPanel>({})", self.should_loop)
    }
}

impl Default for PlayPanel {
    fn default() -> Self {
        Self { playing: false, should_loop: true }
    }
}

impl<'a> View<NotationLayout<'a>> for PlayPanel {
    fn log_set_layout(&self) -> bool {
        true
    }
}

impl PlayPanel {
    pub fn spawn(
        commands: &mut Commands,
        _assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
    ) -> Entity {
        let panel = PlayPanel::default();
        let panel_entity =
            BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(panel));
        ColorBackground::spawn(
            commands,
            panel_entity,
            theme.core.mini_map_z + 1.0,
            theme.colors.of_syllable(Syllable::Do),
        );
        panel_entity
    }
}
