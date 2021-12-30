use fehler::throws;
use notation_proto::prelude::{FrettedEntry4, FrettedEntry6, HandShape4, HandShape6};
use notation_proto::proto_entry::ProtoEntry;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{parenthesized, token, LitInt, Token};

use crate::context::Context;
use crate::core::duration::DurationTweakDsl;

pub struct ShapeDsl {
    pub barre: Option<u8>,
    pub frets: Vec<Option<u8>>,
    pub duration_tweak: Option<DurationTweakDsl>,
}

impl Parse for ShapeDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mut frets = vec![];
        if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            while content.peek(LitInt) || content.peek(Token![_]) {
                if content.peek(LitInt) {
                    frets.push(Some(content.parse::<LitInt>()?.base10_parse::<u8>()?));
                } else {
                    content.parse::<Token![_]>()?;
                    frets.push(None);
                }
            }
            frets.reverse();
        }
        let mut barre = None;
        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            barre = Some(input.parse::<LitInt>()?.base10_parse::<u8>()?);
        }

        let duration_tweak = DurationTweakDsl::try_parse(input);
        ShapeDsl {
            barre,
            frets,
            duration_tweak,
        }
    }
}

impl ToTokens for ShapeDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ShapeDsl {
            barre,
            frets,
            duration_tweak,
        } = self;
        //let string_num = Context::fretted().string_num;
        let mut frets_quote: Vec<TokenStream> = vec![];
        let mut fingers_quote: Vec<TokenStream> = vec![];
        for fret in frets {
            frets_quote.push(match fret {
                Some(fret) => quote! { Some(#fret) },
                None => quote! { None },
            });
            fingers_quote.push(quote! { None });
        }
        let duration_quote = Context::duration_quote(duration_tweak);
        let fretted_entry_quote = Context::fretted().fretted_entry_quote();
        let hand_shape_quote = Context::fretted().hand_shape_quote();
        let barre = barre.unwrap_or(0);
        tokens.extend(quote! {
            ProtoEntry::from(#fretted_entry_quote::from(
                (#hand_shape_quote::new_barre(
                    #barre, [
                    #(#frets_quote),*
                ], [
                    #(#fingers_quote),*
                ]), #duration_quote)
            ))
        });
    }
}

impl ShapeDsl {
    pub fn to_proto(&self) -> ProtoEntry {
        let ShapeDsl {
            barre,
            frets: _,
            duration_tweak,
        } = self;
        let barre = barre.unwrap_or(0);
        let duration = Context::tweaked_duration(duration_tweak);
        match Context::fretted().string_num {
            4 => {
                let mut frets = [None; 4];
                for i in 0..4 {
                    frets[i] = self.frets.get(i).unwrap().clone();
                }
                let fingers = [None; 4];
                ProtoEntry::from(FrettedEntry4::from((
                    HandShape4::new_barre(barre, frets, fingers),
                    duration,
                )))
            }
            _ => {
                let mut frets = [None; 6];
                for i in 0..6 {
                    frets[i] = self.frets.get(i).unwrap().clone();
                }
                let fingers = [None; 6];
                ProtoEntry::from(FrettedEntry6::from((
                    HandShape6::new_barre(barre, frets, fingers),
                    duration,
                )))
            }
        }
    }
}
