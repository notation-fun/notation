use crate::prelude::{Duration, Units};

#[derive(Copy, Clone, Debug)]
pub enum EntryPassMode {
    Immediate,
    Delayed,
}

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
    fn pass_mode(&self) -> EntryPassMode {
        EntryPassMode::Immediate
    }
}
