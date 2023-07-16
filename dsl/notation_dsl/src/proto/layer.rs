use fehler::throws;

use notation_proto::prelude::BarLayer;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};

use crate::proto::slice::SliceDsl;

use super::id::IdDsl;

pub struct LayerDsl {
    pub track: IdDsl,
    pub slices: Vec<SliceDsl>,
}

mod kw {
    syn::custom_keyword!(Rounds);
}

impl LayerDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let track = input.parse()?;
        let slices = SliceDsl::parse_vec(input)?;
        LayerDsl { track, slices }
    }
    pub fn peek(input: ParseStream) -> bool {
        IdDsl::peek(input)
    }
}
impl ToTokens for LayerDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let LayerDsl { track, slices } = self;
        let slices_quote = SliceDsl::quote_vec(slices);
        tokens.extend(quote! {
            BarLayer::new(#track.into(), #slices_quote)
        });
    }
}

impl LayerDsl {
    pub fn to_proto(&self) -> BarLayer {
        let slices = self.slices.iter().map(|x| x.to_proto()).collect();
        BarLayer::new(self.track.id.clone(), slices)
    }
}
