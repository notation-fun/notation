use fehler::throws;
use notation_proto::prelude::{CoreEntry, Note, Tone};
use notation_proto::proto_entry::ProtoEntry;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};

use crate::context::Context;

use super::duration::DurationTweakDsl;
use super::empty::EmptyDsl;
use super::note::NoteDsl;

pub struct ToneDsl {
    pub empty: Option<EmptyDsl>,
    pub notes: Vec<NoteDsl>,
    pub duration_tweak: Option<DurationTweakDsl>,
}

impl ToneDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, multied: bool, with_paren: bool) -> Self {
        let mut empty = None;
        let mut notes = vec![];
        if EmptyDsl::peek(input) {
            empty = Some(input.parse()?);
        } else {
            while NoteDsl::peek(input) {
                notes.push(input.parse()?);
                if multied && !with_paren {
                    break;
                }
            }
        }
        let duration_tweak = DurationTweakDsl::try_parse(input);
        ToneDsl {
            empty,
            notes,
            duration_tweak,
        }
    }
}

impl ToTokens for ToneDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ToneDsl {
            empty,
            notes,
            duration_tweak,
        } = self;
        let duration_quote = Context::duration_quote(duration_tweak);
        if empty.is_some() {
            tokens.extend(empty.as_ref().unwrap().quote(duration_quote));
        } else {
            let notes_quote: Vec<_> = notes.iter().map(|x| quote! { #x }).collect();
            tokens.extend(quote! {
                ProtoEntry::from(CoreEntry::from(
                    (Tone::from(vec![
                        #(#notes_quote),*
                    ]), #duration_quote)
                ))
            });
        }
    }
}

impl ToneDsl {
    pub fn to_proto(&self) -> ProtoEntry {
        let notes = self
            .notes
            .iter()
            .map(|x| x.to_proto())
            .collect::<Vec<Note>>();
        ProtoEntry::from(CoreEntry::from((
            Tone::from(notes),
            Context::tweaked_duration(&self.duration_tweak),
        )))
    }
}
