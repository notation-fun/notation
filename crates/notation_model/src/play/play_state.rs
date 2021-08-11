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
