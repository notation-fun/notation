use std::cmp::{max, min};

use fehler::throws;
use notation_proto::prelude::{Octave, Semitones};
use syn::parse::{Parse, ParseStream};
use syn::{Error, Token};

#[derive(Debug)]
pub struct OctaveTweakDsl {
    pub offset: i8,
}

impl Parse for OctaveTweakDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mut offset: i8 = 0;
        while Self::peek(input) {
            if input.peek(Token![.]) {
                input.parse::<Token![.]>()?;
                offset -= 1;
            } else {
                input.parse::<Token![^]>()?;
                offset += 1;
            }
        }
        offset = min(max(offset, -4), 4);
        OctaveTweakDsl { offset }
    }
}

impl OctaveTweakDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![.]) || input.peek(Token![^])
    }
    pub fn try_parse(input: ParseStream) -> Option<Self> {
        if Self::peek(input) {
            Self::parse(input).ok()
        } else {
            None
        }
    }
    pub fn tweak(&self, base: &Octave) -> Octave {
        let semitones = Semitones::from(*base);
        Octave::from(semitones + Semitones(self.offset * 12))
    }
}
