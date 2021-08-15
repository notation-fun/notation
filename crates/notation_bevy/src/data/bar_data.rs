use notation_model::prelude::{TabBar, TabBarProps, TabPosition, Units};

#[derive(Clone, Debug)]
pub struct BarData<T: Send + Sync + 'static> {
    pub bar_props: TabBarProps,
    pub value: T,
}

impl<T: Send + Sync + 'static> BarData<T> {
    pub fn new(bar: &TabBar, value: T) -> Self {
        Self {
            bar_props: bar.props,
            value,
        }
    }
    pub fn tab_position(&self) -> TabPosition {
        TabPosition::new(Units(
            (self.bar_props.bar_ordinal - 1) as f32 * self.bar_props.bar_units.0,
        ))
    }
}
