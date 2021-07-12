use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitInt, Token};

use crate::context::Context;

pub struct ShapeDsl {
    pub frets: Vec<Option<u8>>,
}

impl Parse for ShapeDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mut frets = vec![];
        while input.peek(LitInt) || input.peek(Token![_]) {
            if input.peek(LitInt) {
                frets.push(Some(input.parse::<LitInt>()?.base10_parse::<u8>()?));
            } else {
                frets.push(None);
            }
        }
        frets.reverse();
        ShapeDsl { frets }
    }
}

impl ToTokens for ShapeDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ShapeDsl { frets } = self;
        let string_num = Context::fretted().string_num;
        let mut frets_quote: Vec<TokenStream> = vec![];
        let mut fingers_quote: Vec<TokenStream> = vec![];
        for fret in frets {
            frets_quote.push(match fret {
                Some(fret) => quote! { Some(#fret) },
                None => quote! { None },
            });
            fingers_quote.push(quote! { None });
        }
        let duration = Context::duration_quote();
        tokens.extend(quote! {
            ProtoEntry::from(FrettedEntry::<#string_num>::from(
                (HandShape::<#string_num>::new([
                    #(#frets_quote),*
                ], [
                    #(#fingers_quote),*
                ]), #duration)
            ))
        });
    }
}
