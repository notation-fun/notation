use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{bracketed, token, LitStr};

pub struct BarDsl {
    pub layers: Vec<LitStr>,
}

impl BarDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let mut layers = Vec::new();
        if input.peek(token::Bracket) {
            let content;
            bracketed!(content in *input);
            while !content.is_empty() {
                layers.push(content.parse()?);
            }
        }
        BarDsl { layers }
    }
}
impl ToTokens for BarDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BarDsl { layers } = self;
        tokens.extend(quote! {
            Bar::from(vec![
                #(#layers),*
            ])
        });
    }
}
