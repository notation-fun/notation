use std::rc::Rc;
use std::sync::Arc;

use notation_core::duration::Duration;

pub trait Entry {
    fn duration(&self) -> Duration {
        Duration::Zero
    }
}

macro_rules! impl_entry_ref {
    ($ref_type:ident) => {
        impl Entry for $ref_type<dyn Entry> {
            fn duration(&self) -> Duration {
                self.as_ref().duration()
            }
        }
    }
}

impl_entry_ref!(Box);
impl_entry_ref!(Rc);
impl_entry_ref!(Arc);
