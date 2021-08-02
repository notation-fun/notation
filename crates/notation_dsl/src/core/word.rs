use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{LitStr, Token};

use crate::context::Context;

use super::duration::DurationTweakDsl;

pub struct WordDsl {
    pub word: Option<LitStr>,
    pub duration_tweak: Option<DurationTweakDsl>,
}

impl WordDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, _multied: bool, _with_paren: bool) -> Self {
        let mut word = None;
        if input.peek(Token![_]) {
            input.parse::<Token![_]>()?;
        } else {
            word = Some(input.parse()?);
        }
        let duration_tweak = DurationTweakDsl::try_parse(input);
        WordDsl {
            word,
            duration_tweak,
        }
    }
}

impl ToTokens for WordDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let WordDsl {
            word,
            duration_tweak,
        } = self;
        let duration_quote = Context::duration_quote(duration_tweak);
        if word.is_none() {
            tokens.extend(quote! {
                ProtoEntry::from(CoreEntry::from(#duration_quote))
            });
        } else {
            let word = word.as_ref().unwrap();
            tokens.extend(quote! {
                ProtoEntry::from((#word, #duration_quote))
            });
        }
    }
}
