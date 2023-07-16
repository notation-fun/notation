use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PlayState {
    Playing,
    Paused,
    Stopped,
}
impl PlayState {
    /// Returns `true` if the play_state is [`Playing`].
    pub fn is_playing(&self) -> bool {
        matches!(self, Self::Playing)
    }
    /// Returns `true` if the play_state is [`Paused`].
    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Paused)
    }
    /// Returns `true` if the play_state is [`Stopped`].
    pub fn is_stopped(&self) -> bool {
        matches!(self, Self::Stopped)
    }
}
impl Default for PlayState {
    fn default() -> Self {
        Self::Stopped
    }
}
impl Display for PlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PlayingState {
    Idle,
    Current,
    Played,
}
impl PlayingState {
    /// Returns `true` if the entry_state is [`Idle`].
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns `true` if the entry_state is [`Playing`].
    pub fn is_current(&self) -> bool {
        matches!(self, Self::Current)
    }

    /// Returns `true` if the entry_state is [`Played`].
    pub fn is_played(&self) -> bool {
        matches!(self, Self::Played)
    }
}
impl Default for PlayingState {
    fn default() -> Self {
        Self::Idle
    }
}
impl Display for PlayingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
