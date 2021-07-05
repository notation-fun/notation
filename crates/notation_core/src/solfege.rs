use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::{Octave, Semitones};

// https://en.wikipedia.org/wiki/Solf%C3%A8ge
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Syllable {
    Do,
    Re,
    Mi,
    Fa,
    So,
    La,
    Ti,
    Di,
    Ri,
    Fi,
    Si,
    Li,
    Ra,
    Me,
    Se,
    Le,
    Te,
}

impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Solfege {
    pub syllable: Syllable,
    pub octave: Octave,
}

impl Display for Solfege {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.syllable, self.octave)
    }
}

impl Solfege {
    pub fn new(syllable: Syllable, octave: Octave) -> Self {
        Self { syllable, octave }
    }
}

impl Solfege {
    pub const DO_0: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P0,
    };
    pub const RE_0: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P0,
    };
    pub const MI_0: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P0,
    };
    pub const FA_0: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P0,
    };
    pub const SO_0: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P0,
    };
    pub const LA_0: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P0,
    };
    pub const TI_0: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P0,
    };

    pub const DI_0: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P0,
    };
    pub const RI_0: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P0,
    };
    pub const FI_0: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P0,
    };
    pub const SI_0: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P0,
    };
    pub const LI_0: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P0,
    };

    pub const RA_0: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P0,
    };
    pub const ME_0: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P0,
    };
    pub const SE_0: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P0,
    };
    pub const LE_0: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P0,
    };
    pub const TE_0: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P0,
    };

    pub const DO_1: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P1,
    };
    pub const RE_1: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P1,
    };
    pub const MI_1: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P1,
    };
    pub const FA_1: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P1,
    };
    pub const SO_1: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P1,
    };
    pub const LA_1: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P1,
    };
    pub const TI_1: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P1,
    };

    pub const DI_1: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P1,
    };
    pub const RI_1: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P1,
    };
    pub const FI_1: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P1,
    };
    pub const SI_1: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P1,
    };
    pub const LI_1: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P1,
    };

    pub const RA_1: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P1,
    };
    pub const ME_1: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P1,
    };
    pub const SE_1: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P1,
    };
    pub const LE_1: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P1,
    };
    pub const TE_1: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P1,
    };

    pub const DO_2: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P2,
    };
    pub const RE_2: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P2,
    };
    pub const MI_2: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P2,
    };
    pub const FA_2: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P2,
    };
    pub const SO_2: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P2,
    };
    pub const LA_2: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P2,
    };
    pub const TI_2: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P2,
    };

    pub const DI_2: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P2,
    };
    pub const RI_2: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P2,
    };
    pub const FI_2: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P2,
    };
    pub const SI_2: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P2,
    };
    pub const LI_2: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P2,
    };

    pub const RA_2: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P2,
    };
    pub const ME_2: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P2,
    };
    pub const SE_2: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P2,
    };
    pub const LE_2: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P2,
    };
    pub const TE_2: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P2,
    };

    pub const DO_3: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P3,
    };
    pub const RE_3: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P3,
    };
    pub const MI_3: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P3,
    };
    pub const FA_3: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P3,
    };
    pub const SO_3: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P3,
    };
    pub const LA_3: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P3,
    };
    pub const TI_3: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P3,
    };

    pub const DI_3: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P3,
    };
    pub const RI_3: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P3,
    };
    pub const FI_3: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P3,
    };
    pub const SI_3: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P3,
    };
    pub const LI_3: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P3,
    };

    pub const RA_3: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P3,
    };
    pub const ME_3: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P3,
    };
    pub const SE_3: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P3,
    };
    pub const LE_3: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P3,
    };
    pub const TE_3: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P3,
    };

    pub const DO_4: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P4,
    };
    pub const RE_4: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P4,
    };
    pub const MI_4: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P4,
    };
    pub const FA_4: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P4,
    };
    pub const SO_4: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P4,
    };
    pub const LA_4: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P4,
    };
    pub const TI_4: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P4,
    };

    pub const DI_4: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P4,
    };
    pub const RI_4: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P4,
    };
    pub const FI_4: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P4,
    };
    pub const SI_4: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P4,
    };
    pub const LI_4: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P4,
    };

    pub const RA_4: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P4,
    };
    pub const ME_4: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P4,
    };
    pub const SE_4: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P4,
    };
    pub const LE_4: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P4,
    };
    pub const TE_4: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P4,
    };

    pub const DO_5: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P5,
    };
    pub const RE_5: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P5,
    };
    pub const MI_5: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P5,
    };
    pub const FA_5: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P5,
    };
    pub const SO_5: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P5,
    };
    pub const LA_5: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P5,
    };
    pub const TI_5: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P5,
    };

    pub const DI_5: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P5,
    };
    pub const RI_5: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P5,
    };
    pub const FI_5: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P5,
    };
    pub const SI_5: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P5,
    };
    pub const LI_5: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P5,
    };

    pub const RA_5: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P5,
    };
    pub const ME_5: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P5,
    };
    pub const SE_5: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P5,
    };
    pub const LE_5: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P5,
    };
    pub const TE_5: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P5,
    };

    pub const DO_6: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P6,
    };
    pub const RE_6: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P6,
    };
    pub const MI_6: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P6,
    };
    pub const FA_6: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P6,
    };
    pub const SO_6: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P6,
    };
    pub const LA_6: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P6,
    };
    pub const TI_6: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P6,
    };

    pub const DI_6: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P6,
    };
    pub const RI_6: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P6,
    };
    pub const FI_6: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P6,
    };
    pub const SI_6: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P6,
    };
    pub const LI_6: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P6,
    };

    pub const RA_6: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P6,
    };
    pub const ME_6: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P6,
    };
    pub const SE_6: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P6,
    };
    pub const LE_6: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P6,
    };
    pub const TE_6: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P6,
    };

    pub const DO_7: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P7,
    };
    pub const RE_7: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P7,
    };
    pub const MI_7: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P7,
    };
    pub const FA_7: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P7,
    };
    pub const SO_7: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P7,
    };
    pub const LA_7: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P7,
    };
    pub const TI_7: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P7,
    };

    pub const DI_7: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P7,
    };
    pub const RI_7: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P7,
    };
    pub const FI_7: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P7,
    };
    pub const SI_7: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P7,
    };
    pub const LI_7: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P7,
    };

    pub const RA_7: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P7,
    };
    pub const ME_7: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P7,
    };
    pub const SE_7: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P7,
    };
    pub const LE_7: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P7,
    };
    pub const TE_7: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P7,
    };

    pub const DO_8: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P8,
    };
    pub const RE_8: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P8,
    };
    pub const MI_8: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P8,
    };
    pub const FA_8: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P8,
    };
    pub const SO_8: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P8,
    };
    pub const LA_8: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P8,
    };
    pub const TI_8: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P8,
    };

    pub const DI_8: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P8,
    };
    pub const RI_8: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P8,
    };
    pub const FI_8: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P8,
    };
    pub const SI_8: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P8,
    };
    pub const LI_8: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P8,
    };

    pub const RA_8: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P8,
    };
    pub const ME_8: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P8,
    };
    pub const SE_8: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P8,
    };
    pub const LE_8: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P8,
    };
    pub const TE_8: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P8,
    };

    pub const DO_9: Self = Self {
        syllable: Syllable::Do,
        octave: Octave::P9,
    };
    pub const RE_9: Self = Self {
        syllable: Syllable::Re,
        octave: Octave::P9,
    };
    pub const MI_9: Self = Self {
        syllable: Syllable::Mi,
        octave: Octave::P9,
    };
    pub const FA_9: Self = Self {
        syllable: Syllable::Fa,
        octave: Octave::P9,
    };
    pub const SO_9: Self = Self {
        syllable: Syllable::So,
        octave: Octave::P9,
    };
    pub const LA_9: Self = Self {
        syllable: Syllable::La,
        octave: Octave::P9,
    };
    pub const TI_9: Self = Self {
        syllable: Syllable::Ti,
        octave: Octave::P9,
    };

    pub const DI_9: Self = Self {
        syllable: Syllable::Di,
        octave: Octave::P9,
    };
    pub const RI_9: Self = Self {
        syllable: Syllable::Ri,
        octave: Octave::P9,
    };
    pub const FI_9: Self = Self {
        syllable: Syllable::Fi,
        octave: Octave::P9,
    };
    pub const SI_9: Self = Self {
        syllable: Syllable::Si,
        octave: Octave::P9,
    };
    pub const LI_9: Self = Self {
        syllable: Syllable::Li,
        octave: Octave::P9,
    };

    pub const RA_9: Self = Self {
        syllable: Syllable::Ra,
        octave: Octave::P9,
    };
    pub const ME_9: Self = Self {
        syllable: Syllable::Me,
        octave: Octave::P9,
    };
    pub const SE_9: Self = Self {
        syllable: Syllable::Se,
        octave: Octave::P9,
    };
    pub const LE_9: Self = Self {
        syllable: Syllable::Le,
        octave: Octave::P9,
    };
    pub const TE_9: Self = Self {
        syllable: Syllable::Te,
        octave: Octave::P9,
    };
}

impl From<Syllable> for Semitones {
    fn from(v: Syllable) -> Self {
        match v {
            Syllable::Do => 0,
            Syllable::Re => 2,
            Syllable::Mi => 4,
            Syllable::Fa => 5,
            Syllable::So => 7,
            Syllable::La => 9,
            Syllable::Ti => 11,
            Syllable::Di => 1,
            Syllable::Ri => 3,
            Syllable::Fi => 6,
            Syllable::Si => 8,
            Syllable::Li => 10,
            Syllable::Ra => 1,
            Syllable::Me => 3,
            Syllable::Se => 6,
            Syllable::Le => 8,
            Syllable::Te => 10,
        }
        .into()
    }
}

impl From<Semitones> for Syllable {
    fn from(v: Semitones) -> Self {
        let pos_val = if v.0 >= 0 { v.0 % 12 } else { v.0 % 12 + 12 };
        match pos_val {
            0 => Syllable::Do,
            1 => Syllable::Di,
            2 => Syllable::Re,
            3 => Syllable::Ri,
            4 => Syllable::Mi,
            5 => Syllable::Fa,
            6 => Syllable::Fi,
            7 => Syllable::So,
            8 => Syllable::Si,
            9 => Syllable::La,
            10 => Syllable::Li,
            11 => Syllable::Ti,
            _ => Syllable::Do,
        }
    }
}

impl From<Semitones> for Solfege {
    fn from(v: Semitones) -> Self {
        let syllable = Syllable::from(v);
        let octave = Octave::from(v);
        Solfege::new(syllable, octave)
    }
}

impl From<Syllable> for Solfege {
    fn from(syllable: Syllable) -> Self {
        Self::new(syllable, Octave::CENTER)
    }
}

impl From<Solfege> for Semitones {
    fn from(v: Solfege) -> Self {
        Semitones::from(v.syllable) + Semitones::from(v.octave)
    }
}
