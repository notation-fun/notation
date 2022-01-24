use bevy::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutHAnchor {
    Left,
    Center,
    Right,
}
impl Display for LayoutHAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for LayoutHAnchor {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutVAnchor {
    Top,
    Center,
    Bottom,
}
impl Display for LayoutVAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for LayoutVAnchor {
    fn default() -> Self {
        Self::Top
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LayoutAnchor {
    pub v: LayoutVAnchor,
    pub h: LayoutHAnchor,
}
impl Display for LayoutAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.v, self.h)
    }
}
impl LayoutHAnchor {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Center => Self::Center,
            Self::Right => Self::Left,
        }
    }
    pub fn factor(&self) -> f32 {
        match self {
            Self::Center => 0.0,
            Self::Left => -0.5,
            Self::Right => 0.5,
        }
    }
}
impl LayoutVAnchor {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Center => Self::Center,
            Self::Bottom => Self::Top,
        }
    }
    pub fn factor(&self) -> f32 {
        match self {
            Self::Center => 0.0,
            Self::Top => 0.5,
            Self::Bottom => -0.5,
        }
    }
}
impl LayoutAnchor {
    pub fn new(v: LayoutVAnchor, h: LayoutHAnchor) -> Self {
        Self { h, v }
    }
    pub fn opposite(&self) -> Self {
        Self::new(self.v.opposite(), self.h.opposite())
    }
    pub fn factor(&self) -> Vec2 {
        Vec2::new(self.h.factor(), self.v.factor())
    }
    pub const ROOT: Self = Self {
        v: LayoutVAnchor::Center,
        h: LayoutHAnchor::Center,
    };
    pub const CENTER: Self = Self {
        v: LayoutVAnchor::Center,
        h: LayoutHAnchor::Center,
    };
    pub const LEFT: Self = Self {
        v: LayoutVAnchor::Center,
        h: LayoutHAnchor::Left,
    };
    pub const RIGHT: Self = Self {
        v: LayoutVAnchor::Center,
        h: LayoutHAnchor::Right,
    };
    pub const TOP: Self = Self {
        v: LayoutVAnchor::Top,
        h: LayoutHAnchor::Center,
    };
    pub const TOP_LEFT: Self = Self {
        v: LayoutVAnchor::Top,
        h: LayoutHAnchor::Left,
    };
    pub const TOP_RIGHT: Self = Self {
        v: LayoutVAnchor::Top,
        h: LayoutHAnchor::Right,
    };
    pub const BOTTOM: Self = Self {
        v: LayoutVAnchor::Bottom,
        h: LayoutHAnchor::Center,
    };
    pub const BOTTOM_LEFT: Self = Self {
        v: LayoutVAnchor::Bottom,
        h: LayoutHAnchor::Left,
    };
    pub const BOTTOM_RIGHT: Self = Self {
        v: LayoutVAnchor::Bottom,
        h: LayoutHAnchor::Right,
    };
}
