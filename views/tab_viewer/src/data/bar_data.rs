use std::fmt::Display;
use bevy::prelude::*;

use notation_model::prelude::{TabBar, TabBarProps, TabPosition, Units};

#[derive(Clone, Debug, Component)]
pub struct BarData<T: Send + Sync + 'static> {
    pub bar_props: TabBarProps,
    pub value: T,
}
impl<T: Send + Sync + ToString + 'static> Display for BarData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BarData<{}>({}: {})",
            std::any::type_name::<T>(),
            self.bar_props.bar_ordinal,
            self.value.to_string()
        )
    }
}
impl<T: Send + Sync + 'static> From<(TabBarProps, T)> for BarData<T> {
    fn from(v: (TabBarProps, T)) -> Self {
        Self {
            bar_props: v.0,
            value: v.1,
        }
    }
}
impl<T: Send + Sync + 'static> BarData<T> {
    pub fn new(bar: &TabBar, value: T) -> Self {
        (bar.props, value).into()
    }
    pub fn tab_position(&self) -> TabPosition {
        TabPosition::new(Units(
            self.bar_props.bar_ordinal as f32 * self.bar_props.bar_units.0,
        ))
    }
}
