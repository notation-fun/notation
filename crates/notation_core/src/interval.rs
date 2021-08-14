use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::Semitones;

// https://hellomusictheory.com/learn/intervals/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
    Tritone,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Interval {
    Unison,
    Minor2nd,
    Major2nd,
    Augmented2nd,
    Minor3nd,
    Major3nd,
    Diminished4th,
    Perfect4th,
    Tritone,
    Perfect5th,
    Augmented5th,
    Minor6th,
    Major6th,
    Minor7th,
    Major7th,
    Perfect8ve,
}
impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_text())
    }
}

impl From<Interval> for IntervalQuality {
    fn from(v: Interval) -> Self {
        match v {
            Interval::Unison => Self::Perfect,
            Interval::Minor2nd => Self::Minor,
            Interval::Major2nd => Self::Minor,
            Interval::Augmented2nd => Self::Augmented,
            Interval::Minor3nd => Self::Minor,
            Interval::Major3nd => Self::Major,
            Interval::Diminished4th => Self::Diminished,
            Interval::Perfect4th => Self::Perfect,
            Interval::Tritone => Self::Tritone,
            Interval::Perfect5th => Self::Perfect,
            Interval::Augmented5th => Self::Augmented,
            Interval::Minor6th => Self::Minor,
            Interval::Major6th => Self::Major,
            Interval::Minor7th => Self::Minor,
            Interval::Major7th => Self::Major,
            Interval::Perfect8ve => Self::Perfect,
        }
    }
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
            Interval::Tritone => 6,
            Interval::Perfect5th => 7,
            Interval::Augmented5th => 8,
            Interval::Minor6th => 8,
            Interval::Major6th => 9,
            Interval::Minor7th => 10,
            Interval::Major7th => 11,
            Interval::Perfect8ve => 12,
        }
        .into()
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
            6 => Self::Tritone,
            7 => Self::Perfect5th,
            8 => Self::Minor6th,
            9 => Self::Major6th,
            10 => Self::Minor7th,
            11 => Self::Major7th,
            _ => Self::Unison,
        }
    }
}
impl Interval {
    pub fn to_text(&self) -> String {
        match self {
            Interval::Unison => "1",
            Interval::Minor2nd => "2-",
            Interval::Major2nd => "2",
            Interval::Augmented2nd => "2+",
            Interval::Minor3nd => "3-",
            Interval::Major3nd => "3",
            Interval::Diminished4th => "4o",
            Interval::Perfect4th => "4",
            Interval::Tritone => "t",
            Interval::Perfect5th => "5",
            Interval::Augmented5th => "5+",
            Interval::Minor6th => "6-",
            Interval::Major6th => "6",
            Interval::Minor7th => "7-",
            Interval::Major7th => "7",
            Interval::Perfect8ve => "8",
        }
        .into()
    }
    pub fn from_text(text: &str) -> Self {
        match text {
            "1" => Self::Unison,
            "2-" => Self::Minor2nd,
            "2" => Self::Major2nd,
            "2+" => Self::Augmented2nd,
            "3-" => Self::Minor3nd,
            "3" => Self::Major3nd,
            "4o" => Self::Diminished4th,
            "4" => Self::Perfect4th,
            "t" => Self::Tritone,
            "5" => Self::Perfect5th,
            "5+" => Self::Perfect5th,
            "5+" => Self::Augmented5th,
            "6-" => Self::Minor6th,
            "6" => Self::Major6th,
            "7-" => Self::Minor7th,
            "7" => Self::Major7th,
            "8" => Self::Perfect8ve,
            _ => Self::Tritone,
        }
    }
}

