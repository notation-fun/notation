use std::fmt::Display;

use notation_core::prelude::Duration;
use serde::{Deserialize, Serialize};

use crate::prelude::Units;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TabPosition {
    pub in_tab_pos: Units,
}
impl Display for TabPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabPosition>({})", self.in_tab_pos.0)
    }
}
impl TabPosition {
    pub fn new(in_tab_pos: Units) -> Self {
        Self { in_tab_pos }
    }
}
impl From<TabPosition> for Units {
    fn from(v: TabPosition) -> Self {
        v.in_tab_pos
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct BarPosition {
    pub bar_units: Units,
    pub bar_ordinal: usize,
    pub in_bar_pos: Units,
}
impl Display for BarPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<BarPosition>({}:{})",
            self.bar_ordinal, self.in_bar_pos.0
        )
    }
}
impl From<BarPosition> for Units {
    fn from(v: BarPosition) -> Self {
        Units(v.bar_ordinal as f32 * v.bar_units.0 + v.in_bar_pos.0)
    }
}
impl BarPosition {
    pub fn new(bar_units: Units, bar_ordinal: usize, in_bar_pos: Units) -> Self {
        Self {
            bar_units,
            bar_ordinal,
            in_bar_pos,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Position {
    pub tab: TabPosition,
    pub bar: BarPosition,
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Position>(tab:{}, bar:{}:{})",
            self.tab.in_tab_pos.0, self.bar.bar_ordinal, self.bar.in_bar_pos.0
        )
    }
}
impl From<Position> for Units {
    fn from(v: Position) -> Self {
        v.tab.in_tab_pos
    }
}

impl Position {
    pub fn new(bar_units: Units) -> Self {
        Self {
            tab: TabPosition::new(Units(0.0)),
            bar: BarPosition::new(bar_units, 1, Units(0.0)),
        }
    }
    pub fn calc_bar_ordinal(&self, pos: Units) -> usize {
        let bar = pos.0 / self.bar.bar_units.0;
        bar.trunc() as usize + 1
    }
    pub fn cal_bar_pos(&self, bar_ordinal: usize) -> Units {
        Units((bar_ordinal - 1) as f32 * self.bar.bar_units.0)
    }
    pub fn tick(&mut self, delta_units: Units) {
        self.set_in_tab(self.tab.in_tab_pos + delta_units);
    }
    pub fn set_in_tab(&mut self, pos: Units) {
        self.tab = TabPosition::new(pos);
        self.bar.bar_ordinal = self.calc_bar_ordinal(pos);
        self.bar.in_bar_pos = self.tab.in_tab_pos - self.cal_bar_pos(self.bar.bar_ordinal);
    }
    pub fn set_in_bar(&mut self, bar_ordinal: usize, in_bar_pos: Units) {
        self.tab = TabPosition::new(self.cal_bar_pos(bar_ordinal) + in_bar_pos);
        self.bar.bar_ordinal = bar_ordinal;
        self.bar.in_bar_pos = in_bar_pos;
    }
    pub fn is_passed(&self, pos: &BarPosition) -> bool {
        let in_tab_pos = self.cal_bar_pos(pos.bar_ordinal) + pos.in_bar_pos;
        in_tab_pos.0 <= self.tab.in_tab_pos.0
    }
    pub fn is_passed_with(&self, pos: &BarPosition, duration: &Duration) -> bool {
        let in_tab_pos =
            self.cal_bar_pos(pos.bar_ordinal) + pos.in_bar_pos + Units::from(*duration);
        in_tab_pos.0 <= self.tab.in_tab_pos.0
    }
}