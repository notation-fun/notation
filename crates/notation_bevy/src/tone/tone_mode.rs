use std::sync::Arc;

use notation_model::prelude::{BarLane, LaneKind};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ToneMode {
    None,
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

impl Default for ToneMode {
    fn default() -> Self {
        Self::None
    }
}

impl From<LaneKind> for ToneMode {
    fn from(v: LaneKind) -> Self {
        match v {
            LaneKind::Melody => Self::Melody,
            LaneKind::Harmany => Self::Harmony,
            LaneKind::Keyboard => Self::Piano,
            _ => Self::None,
        }
    }
}

impl From<Arc<BarLane>> for ToneMode {
    fn from(v: Arc<BarLane>) -> Self {
        Self::from(v.kind)
    }
}

impl From<Option<Arc<BarLane>>> for ToneMode {
    fn from(v: Option<Arc<BarLane>>) -> Self {
        v.map(|x| Self::from(x.kind))
            .unwrap_or_default()
    }
}