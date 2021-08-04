use notation_model::prelude::{Tab, TabBar, TabPosition};
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::prelude::{BarLayout};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub enum LayoutMode {
    Grid,
    Line,
}
impl Default for LayoutMode {
    fn default() -> Self {
        Self::Grid
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LayoutSettings {
    pub mode: LayoutMode,
    pub bars_in_window: u8,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            mode: LayoutMode::default(),
            bars_in_window: 4,
        }
    }
}

impl LayoutSettings {
    fn _calc_bar_layout(
        &self,
        index: usize,
    ) -> BarLayout {
        let row = index / self.bars_in_window as usize;
        let col = index % self.bars_in_window as usize;
        BarLayout::new(row, col)
    }
    pub fn calc_bar_layout(
        &self,
        bar: &TabBar,
    ) -> BarLayout {
        self._calc_bar_layout(bar.bar_ordinal - 1)
    }
    pub fn calc_pos_layout(
        &self,
        tab: &Tab,
        pos: TabPosition,
    ) -> BarLayout {
        let bar_units = tab.bar_units();
        let mut index = (pos.in_tab_pos.0 / bar_units.0) as usize;
        if index >= tab.bars.len() {
            index = tab.bars.len() - 1;
        }
        self._calc_bar_layout(index)
    }
}

