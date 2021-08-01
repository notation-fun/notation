#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ToneMode {
    Melody,
    Harmony,
    Piano,
}

impl ToneMode {
    /// Returns `true` if the tone_mode is [`Melody`].
    pub fn is_melody(&self) -> bool {
        matches!(self, Self::Melody)
    }

    /// Returns `true` if the tone_mode is [`Harmony`].
    pub fn is_harmony(&self) -> bool {
        matches!(self, Self::Harmony)
    }

    /// Returns `true` if the tone_mode is [`Piano`].
    pub fn is_piano(&self) -> bool {
        matches!(self, Self::Piano)
    }
}