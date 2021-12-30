use fehler::throws;
use notation_proto::prelude::PickNote;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitInt, Token};

pub struct PickNoteDsl {
    pub string: u8,
    pub fret: Option<u8>,
}

impl Parse for PickNoteDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let string = input.parse::<LitInt>()?.base10_parse::<u8>()?;
        let fret = if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Some(input.parse::<LitInt>()?.base10_parse::<u8>()?)
        } else {
            None
        };
        PickNoteDsl { string, fret }
    }
}

impl ToTokens for PickNoteDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PickNoteDsl { string, fret } = self;
        let fret_quote = fret.map(|f| quote! { Some(#f) }).unwrap_or(quote! {None});
        tokens.extend(quote! {
            PickNote::new(#string, #fret_quote, None, None, None)
        });
    }
}

impl PickNoteDsl {
    pub fn to_proto(&self) -> PickNote {
        PickNote::new(self.string, self.fret, None, None, None)
    }
}
