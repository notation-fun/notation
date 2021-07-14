use serde::{Deserialize, Serialize};

use super::bevy_theme::BevyTheme;
use super::grid_config::GridConfig;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
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

pub struct BevyConfigAccessor<T> {
    pub calc: fn(&BevyConfig) -> T,
}
