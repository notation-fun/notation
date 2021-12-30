use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
    Bpm(u16),
}
impl Display for Tempo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(bpm) = self.as_bpm() {
            write!(f, "{}", bpm)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
impl Tempo {
    pub fn as_bpm(&self) -> Option<&u16> {
        if let Self::Bpm(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

pub type BpmRange = (u16, u16);
pub type Bpm = u16;

impl From<Tempo> for BpmRange {
    fn from(v: Tempo) -> Self {
        match v {
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
    fn from(v: Tempo) -> Self {
        let range = BpmRange::from(v);
        (range.0 + range.1) / 2
    }
}

impl From<Bpm> for Tempo {
    fn from(v: Bpm) -> Self {
        Self::Bpm(v)
    }
}

impl Tempo {
    pub fn to_ident(&self) -> String {
        format!("{}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "Larghissimo" => Tempo::Larghissimo,
            "Adagissimo" => Tempo::Adagissimo,
            "Grave" => Tempo::Grave,
            "Largo" => Tempo::Largo,
            "Lento" => Tempo::Lento,
            "Larghetto" => Tempo::Larghetto,
            "Adagio" => Tempo::Adagio,
            "Adagietto" => Tempo::Adagietto,
            "Andante" => Tempo::Andante,
            "Andantino" => Tempo::Andantino,
            "MarciaModerato" => Tempo::MarciaModerato,
            "Moderato" => Tempo::Moderato,
            "Allegretto" => Tempo::Allegretto,
            "AllegroModerato" => Tempo::AllegroModerato,
            "Allegro" => Tempo::Allegro,
            "Vivace" => Tempo::Vivace,
            "Vivacissimo" => Tempo::Vivacissimo,
            "Allegrissimo" => Tempo::Allegrissimo,
            "Presto" => Tempo::Presto,
            "Prestissimo" => Tempo::Prestissimo,
            _ => {
                let bpm = ident.parse::<u16>().unwrap();
                Tempo::Bpm(bpm)
            }
        }
    }
}
