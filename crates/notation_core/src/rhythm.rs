use serde::{Serialize, Deserialize};

use crate::prelude::{Unit, Units};

// https://hellomusictheory.com/learn/musical-term-for-slow/
// https://hellomusictheory.com/learn/musical-term-for-fast/

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Tempo {
    Larghissimo,
    Adagissimo,
    Grave,
    Largo,
    Lento,
    Larghetto,
    Adagio,
    Adagietto,
    Andante,
    Andantino,
    MarciaModerato,
    Moderato,
    Allegretto,
    AllegroModerato,
    Allegro,
    Vivace,
    Vivacissimo,
    Allegrissimo,
    Presto,
    Prestissimo,
    Bpm (u8),
}

pub type BpmRange = (u8, u8);
pub type Bpm = u8;

impl From<Tempo> for BpmRange {
    fn from(val: Tempo) -> Self {
        match val {
            Tempo::Larghissimo => (1, 24),
            Tempo::Adagissimo => (20, 40),
            Tempo::Grave => (25, 45),
            Tempo::Largo => (40, 60),
            Tempo::Lento => (45, 60),
            Tempo::Larghetto => (60, 66),
            Tempo::Adagio => (66, 76),
            Tempo::Adagietto => (70, 80),
            Tempo::Andante => (76, 108),
            Tempo::Andantino => (80, 108),
            Tempo::MarciaModerato => (83, 85),
            Tempo::Moderato => (108, 120),
            Tempo::Allegretto => (112, 120),
            Tempo::AllegroModerato => (116, 120),
            Tempo::Allegro => (120, 156),
            Tempo::Vivace => (156, 176),
            Tempo::Vivacissimo => (172, 176),
            Tempo::Allegrissimo => (172, 176),
            Tempo::Presto => (168, 200),
            Tempo::Prestissimo => (200, 255),
            Tempo::Bpm(bpm) => (bpm, bpm),
        }
    }
}

impl From<Tempo> for Bpm {
    fn from(val: Tempo) -> Self {
        let range = BpmRange::from(val);
        (range.0 + range.1) / 2
    }
}

impl From<Bpm> for Tempo {
    fn from(val: Bpm) -> Self {
        Self::Bpm(val)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Signature {
    beat_unit: Unit,
    beats_per_bar: u8,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct Beats(pub f32);

impl Signature {
    pub fn new(beat_unit: Unit, beats_per_bar: u8) -> Self {
        Self {
            beat_unit,
            beats_per_bar,
        }
    }
    pub fn new_4_4() -> Self {
        Self::new(Unit::Quarter, 4)
    }
    pub fn new_3_4() -> Self {
        Self::new(Unit::Quarter, 3)
    }
    pub fn new_2_4() -> Self {
        Self::new(Unit::Quarter, 2)
    }
    pub fn new_6_8() -> Self {
        Self::new(Unit::Eighth, 6)
    }
}

impl From<f32> for Beats {
    fn from(val: f32) -> Self {
        Self(val)
    }
}

impl From<Signature> for Units {
    fn from(v: Signature) -> Self {
        Self::from(Units::from(v.beat_unit).0 * (v.beats_per_bar as f32))
    }
}

impl From<Signature> for Beats {
    fn from(v: Signature) -> Self {
        Self::from(v.beats_per_bar as f32)
    }
}

impl From<(Signature, Units)> for Beats {
    fn from((signature, units): (Signature, Units)) -> Self {
        Self::from(units.0 / Units::from(signature.beat_unit).0)
    }
}