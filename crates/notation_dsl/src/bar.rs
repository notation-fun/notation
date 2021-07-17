use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{LitStr};

pub struct BarDsl {
    pub layers: Vec<LitStr>,
}

impl BarDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, multied: bool, with_paren: bool) -> Self {
        let mut layers = Vec::new();
        while input.peek(LitStr) {
            layers.push(input.parse::<LitStr>()?);
            if multied && !with_paren {
                break;
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
