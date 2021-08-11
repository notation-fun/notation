use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::LitStr;

use crate::context::Context;

use super::duration::DurationTweakDsl;
use super::empty::EmptyDsl;

pub struct WordDsl {
    pub empty: Option<EmptyDsl>,
    pub word: Option<LitStr>,
    pub duration_tweak: Option<DurationTweakDsl>,
}

impl WordDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, _multied: bool, _with_paren: bool) -> Self {
        let mut empty = None;
        let mut word = None;
        if EmptyDsl::peek(input) {
            empty = Some(input.parse()?);
        } else {
            word = Some(input.parse()?);
        }
        let duration_tweak = DurationTweakDsl::try_parse(input);
        WordDsl {
            empty,
            word,
            duration_tweak,
        }
    }
}

impl ToTokens for WordDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let WordDsl {
            empty,
            word,
            duration_tweak,
        } = self;
        let duration_quote = Context::duration_quote(duration_tweak);
        if empty.is_some() {
            tokens.extend(empty.as_ref().unwrap().quote(duration_quote));
        } else {
            let word = word.as_ref().unwrap();
            tokens.extend(quote! {
                ProtoEntry::from((#word, #duration_quote))
            });
        }
    }
}
