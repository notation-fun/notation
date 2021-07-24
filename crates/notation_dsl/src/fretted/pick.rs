use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::LitInt;

use crate::context::Context;

use super::pick_note::PickNoteDsl;

pub struct PickDsl {
    pub notes: Vec<PickNoteDsl>,
}

impl PickDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, multied: bool, with_paren: bool) -> Self {
        let mut notes = vec![];
        while input.peek(LitInt) {
            notes.push(input.parse()?);
            if multied && !with_paren {
                break;
            }
        }
        PickDsl { notes }
    }
}

impl ToTokens for PickDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PickDsl { notes } = self;
        let duration = Context::duration_quote();
        let string_num = Context::fretted().string_num;
        let notes_quote: Vec<_> = notes.iter().map(|x| quote! { #x }).collect();
        tokens.extend(quote! {
            ProtoEntry::from(FrettedEntry::<#string_num>::from(
                (Pick::from(vec![
                    #(#notes_quote),*
                ]), #duration)
            ))
        });
    }
}
