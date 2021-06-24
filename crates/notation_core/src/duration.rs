use std::ops::{Add, Sub};

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Unit {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecondth,
}

impl Default for Unit {
    fn default() -> Self {
        Self::Quarter
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct Units(pub f32);

impl Add for Units {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Units(self.0 + rhs.0)
    }
}

impl Sub for Units {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Units(self.0 - rhs.0)
    }
}
// https://hellomusictheory.com/learn/tuplets/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Duration {
    Zero,
    Simple (Unit),
    Dotted (Unit),
    Triplet (Unit),
    DottedTriplet (Unit),
}

impl Default for Duration {
    fn default() -> Self {
        Self::Simple(Unit::default())
    }
}

impl From<Unit> for Duration {
    fn from(v: Unit) -> Self {
        Self::Simple(v)
    }
}

impl From<f32> for Units {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

impl From<Unit> for Units {
    fn from(v: Unit) -> Self {
        match v {
            Unit::Whole => 1.0,
            Unit::Half => 1.0 / 2.0,
            Unit::Quarter => 1.0 / 4.0,
            Unit::Eighth => 1.0 / 8.0,
            Unit::Sixteenth => 1.0 / 16.0,
            Unit::ThirtySecondth => 1.0 / 32.0,
        }.into()
    }
}

impl From<Duration> for Units {
    fn from(v: Duration) -> Self {
        match v {
            Duration::Zero => 0.0,
            Duration::Simple(v) => Units::from(v).0,
            Duration::Dotted(v) => Units::from(v).0 * 1.5,
            Duration::Triplet(v) => Units::from(v).0 * 2.0 / 3.0,
            Duration::DottedTriplet(v) => Units::from(v).0 * 4.0 / 3.0,
        }.into()
    }
}