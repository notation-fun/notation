use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{LitInt, Token};

use crate::proto::slice::SliceDsl;

use super::id::IdDsl;

pub struct LayerDsl {
    pub id: IdDsl,
    pub slices: Vec<SliceDsl>,
    pub track: Option<IdDsl>,
    pub rounds: Option<Vec<usize>>,
}

mod kw {
    syn::custom_keyword!(Track);
    syn::custom_keyword!(Rounds);
}

impl LayerDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let id = input.parse()?;
        let slices = SliceDsl::parse_vec(input)?;
        let mut track = None;
        let mut rounds = None;
        loop {
            if input.peek(kw::Track) {
                input.parse::<kw::Track>()?;
                input.parse::<Token![:]>()?;
                track = Some(input.parse()?);
            } else if input.peek(kw::Rounds) {
                input.parse::<kw::Rounds>()?;
                input.parse::<Token![:]>()?;
                let mut rounds_: Vec<usize> = Vec::new();
                while input.peek(LitInt) {
                    rounds_.push(input.parse::<LitInt>()?.base10_parse::<usize>()?);
                }
                rounds = Some(rounds_);
            } else {
                break;
            }
        }
        LayerDsl {
            id,
            slices,
            track,
            rounds,
        }
    }
}
impl ToTokens for LayerDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let LayerDsl {
            id,
            slices,
            track,
            rounds,
        } = self;
        let slices_quote = SliceDsl::quote_vec(slices);
        let track_quote = match track {
            Some(track) => quote! { Some(#track.into()) },
            None => quote! { None },
        };
        let rounds_quote = match rounds {
            Some(rounds) => {
                quote! {
                    Some(vec![
                        #(#rounds).*
                    ])
                }
            }
            None => {
                quote! { None }
            }
        };
        tokens.extend(quote! {
            BarLayer::new(#id.into(), #slices_quote, #track_quote, #rounds_quote)
        });
    }
}
