use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{LitInt, LitStr, Token};

use crate::proto::slice::SliceDsl;

pub struct LayerDsl {
    pub key: LitStr,
    pub slices: Vec<SliceDsl>,
    pub track: Option<LitStr>,
    pub rounds: Option<Vec<u16>>,
}

mod kw {
    syn::custom_keyword!(track);
    syn::custom_keyword!(rounds);
}

impl LayerDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let key = input.parse()?;
        let slices = SliceDsl::parse_vec(input)?;
        let mut track = None;
        let mut rounds = None;
        loop {
            if input.peek(kw::track) {
                input.parse::<kw::track>()?;
                input.parse::<Token![:]>()?;
                track = Some(input.parse()?);
            } else if input.peek(kw::rounds) {
                input.parse::<kw::rounds>()?;
                input.parse::<Token![:]>()?;
                let mut rounds_: Vec<u16> = Vec::new();
                while input.peek(LitInt) {
                    rounds_.push(input.parse::<LitInt>()?.base10_parse::<u16>()?);
                }
                rounds = Some(rounds_);
            } else {
                break;
            }
        }
        LayerDsl {
            key,
            slices,
            track,
            rounds,
        }
    }
}
impl ToTokens for LayerDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let LayerDsl {
            key,
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
            BarLayer::new(#key.into(), #slices_quote, #track_quote, #rounds_quote)
        });
    }
}
