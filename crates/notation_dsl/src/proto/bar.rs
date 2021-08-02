use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};

use super::layer::LayerDsl;

pub struct BarDsl {
    pub layers: Vec<LayerDsl>,
}

impl BarDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let mut layers = Vec::new();
        while LayerDsl::peek(input) {
            layers.push(input.parse::<LayerDsl>()?);
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
