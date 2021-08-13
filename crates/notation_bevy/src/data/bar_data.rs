use notation_model::prelude::{TabBar, TabBarProps};

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
}
