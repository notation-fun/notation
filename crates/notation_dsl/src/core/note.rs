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
    pub pitch_name: PitchNameDsl,
    pub pitch_sign: PitchSignDsl,
}

impl Parse for NoteDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let octave_tweak = OctaveTweakDsl::try_parse(input);
        let pitch_name = input.parse::<PitchNameDsl>()?;
        let pitch_sign = input.parse::<PitchSignDsl>()?;
        NoteDsl {
            octave_tweak,
            pitch_name,
            pitch_sign,
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
        let note = self.to_proto();
        let octave_ident = note.octave.to_ident();
        let pitch_text = note.pitch.to_text();
        let syllable_text = note.syllable.to_text();
        tokens.extend(quote! {
            Note::new(Octave::from_ident(#octave_ident), Pitch::from_text(#pitch_text), Syllable::from_text(#syllable_text))
        });
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
            Context::calc_note_from_syllable(octave_tweak, &syllable)
        } else {
            let pitch = Pitch::new(pitch_name.name, pitch_sign.sign);
            Context::calc_note_from_pitch(octave_tweak, &pitch)
        }
    }
}
