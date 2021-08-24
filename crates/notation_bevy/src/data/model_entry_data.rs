use std::fmt::Display;

use notation_model::prelude::{ModelEntry, ModelEntryProps};

#[derive(Clone, Debug)]
pub struct ModelEntryData<T: Send + Sync + 'static> {
    pub entry_props: ModelEntryProps,
    pub value: T,
}
impl<T: Send + Sync + ToString + 'static> Display for ModelEntryData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ModelEntryData<{}>({}: {})",
            std::any::type_name::<T>(),
            self.entry_props.index,
            self.value.to_string()
        )
    }
}
impl<T: Send + Sync + 'static> From<(ModelEntryProps, T)> for ModelEntryData<T> {
    fn from(v: (ModelEntryProps, T)) -> Self {
        Self {
            entry_props: v.0,
            value: v.1,
        }
    }
}
impl<T: Send + Sync + 'static> ModelEntryData<T> {
    pub fn new(entry: &ModelEntry, value: T) -> Self {
        Self {
            entry_props: entry.props,
            value,
        }
    }
}
