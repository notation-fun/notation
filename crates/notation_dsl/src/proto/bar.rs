use fehler::throws;

use notation_proto::prelude::{Bar, BarLayer};
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

impl BarDsl {
    pub fn to_proto(&self) -> Bar {
        self.layers
            .iter()
            .map(|x| x.to_proto())
            .collect::<Vec<BarLayer>>()
            .into()
    }
}
