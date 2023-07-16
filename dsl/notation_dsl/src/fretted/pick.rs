use fehler::throws;
use notation_proto::prelude::{CoreEntry, FrettedEntry4, FrettedEntry6, Pick};
use notation_proto::proto_entry::ProtoEntry;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{LitInt, Token};

use crate::context::Context;
use crate::core::duration::DurationTweakDsl;

use super::pick_note::PickNoteDsl;

pub struct PickDsl {
    pub notes: Vec<PickNoteDsl>,
    pub duration_tweak: Option<DurationTweakDsl>,
}

impl PickDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, multied: bool, with_paren: bool) -> Self {
        let mut notes = vec![];
        if input.peek(Token![_]) {
            input.parse::<Token![_]>()?;
        } else {
            while input.peek(LitInt) {
                notes.push(input.parse()?);
                if multied && !with_paren {
                    break;
                }
            }
        }
        let duration_tweak = DurationTweakDsl::try_parse(input);
        PickDsl {
            notes,
            duration_tweak,
        }
    }
}

impl ToTokens for PickDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PickDsl {
            notes,
            duration_tweak,
        } = self;
        let duration_quote = Context::duration_quote(duration_tweak);
        if notes.len() == 0 {
            tokens.extend(quote! {
                ProtoEntry::from(CoreEntry::from(#duration_quote))
            });
        } else {
            let _string_num = Context::fretted().string_num;
            let notes_quote: Vec<_> = notes.iter().map(|x| quote! { #x }).collect();
            let fretted_entry_quote = Context::fretted().fretted_entry_quote();
            tokens.extend(quote! {
                ProtoEntry::from(#fretted_entry_quote::from(
                    (Pick::from(vec![
                        #(#notes_quote),*
                    ]), #duration_quote)
                ))
            });
        }
    }
}

impl PickDsl {
    pub fn to_proto(&self) -> ProtoEntry {
        let PickDsl {
            notes,
            duration_tweak,
        } = self;
        let duration = Context::tweaked_duration(duration_tweak);
        if notes.len() == 0 {
            ProtoEntry::from(CoreEntry::from(duration))
        } else {
            let _string_num = Context::fretted().string_num;
            let notes: Vec<_> = notes.iter().map(|x| x.to_proto()).collect();
            match Context::fretted().string_num {
                4 => ProtoEntry::from(FrettedEntry4::from((Pick::from(notes), duration))),
                _ => ProtoEntry::from(FrettedEntry6::from((Pick::from(notes), duration))),
            }
        }
    }
}
