use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};

use crate::context::Context;

use super::duration::DurationTweakDsl;
use super::note::NoteDsl;

pub struct ToneDsl {
    pub notes: Vec<NoteDsl>,
    pub duration_tweak: Option<DurationTweakDsl>,
}

impl ToneDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, multied: bool, with_paren: bool) -> Self {
        let mut notes = vec![];
        while NoteDsl::peek(input) {
            notes.push(input.parse()?);
            if multied && !with_paren {
                break;
            }
        }
        let duration_tweak = DurationTweakDsl::try_parse(input);
        ToneDsl { notes, duration_tweak }
    }
}

impl ToTokens for ToneDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ToneDsl { notes, duration_tweak } = self;
        let duration_quote = Context::duration_quote(duration_tweak);
        let string_num = Context::fretted().string_num;
        let notes_quote: Vec<_> = notes.iter().map(|x| quote! { #x }).collect();
        tokens.extend(quote! {
            ProtoEntry::from(CoreEntry::from(
                (Tone::from(vec![
                    #(#notes_quote),*
                ]), #duration_quote)
            ))
        });
    }
}

