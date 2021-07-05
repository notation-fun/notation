use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use super::bevy_theme::BevyTheme;
use super::grid_config::GridConfig;

#[derive(PartialEq, Serialize, Deserialize, Debug, Inspectable)]
pub struct BevyConfig {
    pub grid: GridConfig,
    pub theme: BevyTheme,
}

impl Default for BevyConfig {
    fn default() -> Self {
        Self {
            grid: GridConfig::default(),
            theme: BevyTheme::default(),
        }
    }
}
