use crate::prelude::{Duration, Units};

pub trait Entry {
    fn duration(&self) -> Duration {
        Duration::Zero
    }
    fn prev_is_tie(&self) -> bool {
        false
    }
    fn next_is_tie(&self) -> bool {
        false
    }
    fn tied_units(&self) -> Units {
        self.duration().into()
    }
}
