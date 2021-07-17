use std::fmt::Display;

#[derive(Debug)]
pub enum EntryState {
    Idle,
    Playing,
    Played,
}
impl EntryState {
    /// Returns `true` if the entry_state is [`Idle`].
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns `true` if the entry_state is [`Playing`].
    pub fn is_playing(&self) -> bool {
        matches!(self, Self::Playing)
    }

    /// Returns `true` if the entry_state is [`Played`].
    pub fn is_played(&self) -> bool {
        matches!(self, Self::Played)
    }
}
impl Default for EntryState {
    fn default() -> Self {
        Self::Idle
    }
}
impl Display for EntryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
