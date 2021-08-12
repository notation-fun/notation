use fehler::throws;
use notation_proto::prelude::GUITAR_FRET_NUM_ACOUSTIC;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Expr, LitInt, Token};

use crate::context::Context;

pub struct FretboardDsl {
    pub tuning: Option<Expr>,
    pub fret_num: Option<usize>,
    pub capo: Option<u8>,
}

mod kw {
    syn::custom_keyword!(tuning);
    syn::custom_keyword!(fret_num);
    syn::custom_keyword!(capo);
}

impl Parse for FretboardDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mut fret_num = None;
        let mut tuning = None;
        let mut capo = None;
        loop {
            if input.peek(kw::tuning) {
                input.parse::<kw::tuning>()?;
                input.parse::<Token![:]>()?;
                tuning = Some(input.parse::<Expr>()?);
            } else if input.peek(kw::fret_num) {
                input.parse::<kw::fret_num>()?;
                input.parse::<Token![:]>()?;
                fret_num = Some(input.parse::<LitInt>()?.base10_parse::<usize>()?);
            } else if input.peek(kw::capo) {
                input.parse::<kw::capo>()?;
                input.parse::<Token![:]>()?;
                capo = Some(input.parse::<LitInt>()?.base10_parse::<u8>()?);
            } else {
                break;
            }
        }
        FretboardDsl {
            tuning,
            fret_num,
            capo,
        }
    }
}

impl ToTokens for FretboardDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FretboardDsl {
            tuning,
            fret_num,
            capo,
        } = self;
        let string_num = Context::fretted().string_num;
        let fret_num = fret_num.unwrap_or(match string_num {
            6 => GUITAR_FRET_NUM_ACOUSTIC,
            _ => GUITAR_FRET_NUM_ACOUSTIC,
        });
        let capo = capo.unwrap_or(0);
        let tuning_quote = match tuning {
            Some(tuning) => quote! { #tuning },
            None => match string_num {
                6 => quote! { GuitarTuning::Standard },
                _ => quote! { GuitarTuning::Standard },
            },
        };
        let fretted_entry_quote = Context::fretted().fretted_entry_quote();
        let fretboard_quote = Context::fretted().fretboard_quote();
        tokens.extend(quote! {
            ProtoEntry::from(#fretted_entry_quote::from(
                #fretboard_quote::new(#fret_num, #tuning_quote.into(), #capo)
            ))
        });
    }
}
