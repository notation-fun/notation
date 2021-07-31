use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitChar, LitInt, Token};
use notation_proto::prelude::{Octave, PitchSign, Semitones, Syllable};

use crate::context::Context;
use super::octave::OctaveTweakDsl;

pub struct PitchSignDsl {
    pub sign: PitchSign,
}

impl Parse for PitchSignDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mut semitones: i8 = 0;
        for _ in 0..2 {
            if input.peek::<>(Token![#]) {
                input.parse::<Token![#]>()?;
                semitones += 1;
            } else if input.peek(Token![%]) {
                input.parse::<Token![%]>()?;
                semitones -= 1;
            }
        }
        let sign = PitchSign::from(Semitones(semitones));
        PitchSignDsl { sign }
    }
}

impl PitchSignDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![#])
            || input.peek(Token![%])
    }
}