use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::LitInt;

use crate::context::Context;

pub struct PickDsl {
    pub strings: Vec<u8>,
}

impl Parse for PickDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mut strings = vec![];
        while input.peek(LitInt) {
            strings.push(input.parse::<LitInt>()?.base10_parse::<u8>()?);
        }
        PickDsl { strings }
    }
}

impl ToTokens for PickDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PickDsl { strings } = self;
        let duration = Context::duration_quote();
        let string_num = Context::fretted().string_num;
        let strings_quote: Vec<_> = strings.iter().map(|x| quote! { #x }).collect();
        tokens.extend(quote! {
            ProtoEntry::from(FrettedEntry::<#string_num>::from(
                (Pick::from(vec![
                    #(#strings_quote),*
                ]), #duration)
            ))
        });
    }
}
