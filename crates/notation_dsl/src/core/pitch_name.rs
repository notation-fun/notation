use fehler::{throws, throw};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitChar, LitInt, Token};
use notation_proto::prelude::{Octave, Pitch, PitchName, PitchSign, Semitones};

use crate::context::Context;
use super::octave::OctaveTweakDsl;

mod kw {
    syn::custom_keyword!(C);
    syn::custom_keyword!(D);
    syn::custom_keyword!(E);
    syn::custom_keyword!(F);
    syn::custom_keyword!(G);
    syn::custom_keyword!(A);
    syn::custom_keyword!(B);
    syn::custom_keyword!(b);
}

pub struct PitchNameDsl {
    pub name: PitchName,
    pub from_syllable: bool,
}

impl Parse for PitchNameDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        if input.peek(LitInt) {
            let name = match input.parse::<LitInt>()?.base10_parse::<u8>()? {
                1 => PitchName::C,
                2 => PitchName::D,
                3 => PitchName::E,
                4 => PitchName::F,
                5 => PitchName::G,
                6 => PitchName::A,
                7 => PitchName::B,
                _ => PitchName::C,
            };
            PitchNameDsl { name, from_syllable: true }
        } else {
            let name =
                if input.peek(kw::C) {
                    input.parse::<kw::C>()?;
                    PitchName::C
                } else if input.peek(kw::D) {
                    input.parse::<kw::D>()?;
                    PitchName::D
                } else if input.peek(kw::E) {
                    input.parse::<kw::E>()?;
                    PitchName::E
                } else if input.peek(kw::F) {
                    input.parse::<kw::F>()?;
                    PitchName::F
                } else if input.peek(kw::G) {
                    input.parse::<kw::G>()?;
                    PitchName::G
                } else if input.peek(kw::A) {
                    input.parse::<kw::A>()?;
                    PitchName::A
                } else if input.peek(kw::B) {
                    input.parse::<kw::B>()?;
                    PitchName::B
                } else {
                    throw!(Error::new(input.span(), "Invalid Pitch Name"));
                };
            PitchNameDsl { name, from_syllable: false }
        }
    }
}

impl PitchNameDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(LitInt)
            || input.peek(kw::C)
            || input.peek(kw::D)
            || input.peek(kw::E)
            || input.peek(kw::F)
            || input.peek(kw::G)
            || input.peek(kw::A)
            || input.peek(kw::B)
    }
}