use serde::{Serialize, Deserialize};

use crate::note::Semitones;

// https://hellomusictheory.com/learn/intervals/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Interval {
    Unison,
    Minor2nd, Major2nd, Augmented2nd,
    Minor3nd, Major3nd,
    Diminished4th, Perfect4th, Augmented4th,
    Diminished5th, Perfect5th, Augmented5th,
    Minor6th, Major6th,
    Minor7th, Major7th,
    Perfect8ve,
}

impl From<Interval> for Semitones {
    fn from(v: Interval) -> Self {
        match v {
            Interval::Unison => 0,
            Interval::Minor2nd => 1,
            Interval::Major2nd => 2,
            Interval::Augmented2nd => 3,
            Interval::Minor3nd => 3,
            Interval::Major3nd => 4,
            Interval::Diminished4th => 4,
            Interval::Perfect4th => 5,
            Interval::Augmented4th => 6,
            Interval::Diminished5th => 6,
            Interval::Perfect5th => 7,
            Interval::Augmented5th => 8,
            Interval::Minor6th => 8,
            Interval::Major6th => 9,
            Interval::Minor7th => 10,
            Interval::Major7th => 11,
            Interval::Perfect8ve => 12,
        }.into()
    }
}

impl From<Semitones> for Interval {
    fn from(v: Semitones) -> Self {
        if v.0 == 0 {
            return Self::Unison;
        }
        match v.0.abs() % 12 {
            0 => Self::Perfect8ve,
            1 => Self::Minor2nd,
            2 => Self::Major2nd,
            3 => Self::Minor3nd,
            4 => Self::Major3nd,
            5 => Self::Perfect4th,
            6 => Self::Diminished5th,
            7 => Self::Perfect5th,
            8 => Self::Minor6th,
            9 => Self::Major6th,
            10 => Self::Minor7th,
            11 => Self::Major7th,
            _ => Self::Unison,
        }
    }
}
