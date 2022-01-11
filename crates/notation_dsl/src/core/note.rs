use fehler::throws;
use notation_proto::prelude::{Note, Pitch, Syllable};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};

use crate::context::Context;

use super::octave::OctaveTweakDsl;
use super::pitch_name::PitchNameDsl;
use super::pitch_sign::PitchSignDsl;

pub struct NoteDsl {
    pub octave_tweak: Option<OctaveTweakDsl>,
    pub pitch_sign: PitchSignDsl,
    pub pitch_name: PitchNameDsl,
}

impl Parse for NoteDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let octave_tweak = OctaveTweakDsl::try_parse(input);
        let pitch_sign = input.parse::<PitchSignDsl>()?;
        let pitch_name = input.parse::<PitchNameDsl>()?;
        NoteDsl {
            octave_tweak,
            pitch_sign,
            pitch_name,
        }
    }
}

impl NoteDsl {
    pub fn peek(input: ParseStream) -> bool {
        OctaveTweakDsl::peek(input) || PitchSignDsl::peek(input) || PitchNameDsl::peek(input)
    }
}

impl ToTokens for NoteDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let NoteDsl {
            octave_tweak,
            pitch_sign,
            pitch_name,
        } = self;
        if pitch_name.from_syllable {
            let syllable = Syllable::from((pitch_sign.sign, pitch_name.name));
            let note_quote = Context::calc_note_quote(octave_tweak, &syllable);
            tokens.extend(note_quote);
        } else {
            let octave_quote = Context::octave_quote(octave_tweak);
            let pitch = Pitch::new(pitch_name.name, pitch_sign.sign);
            let pitch_text = pitch.to_text();
            tokens.extend(quote! {
                Note::new(#octave_quote, Pitch::from_text(#pitch_text))
            });
        }
    }
}

impl NoteDsl {
    pub fn to_proto(&self) -> Note {
        let NoteDsl {
            octave_tweak,
            pitch_sign,
            pitch_name,
        } = self;
        if pitch_name.from_syllable {
            let syllable = Syllable::from((pitch_sign.sign, pitch_name.name));
            Context::calc_note(octave_tweak, &syllable)
        } else {
            let octave = Context::tweaked_octave(octave_tweak);
            let pitch = Pitch::new(pitch_name.name, pitch_sign.sign);
            Note::new(octave, pitch)
        }
    }
}
