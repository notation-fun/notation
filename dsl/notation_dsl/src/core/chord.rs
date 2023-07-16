use fehler::{throw, throws};
use notation_proto::prelude::{Chord, CoreEntry, Interval};
use notation_proto::proto_entry::ProtoEntry;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::Token;

use crate::context::Context;

use super::duration::DurationTweakDsl;
use super::empty::EmptyDsl;
use super::interval::IntervalDsl;
use super::syllable::SyllableDsl;

pub struct ChordDsl {
    pub empty: Option<EmptyDsl>,
    pub root: Option<SyllableDsl>,
    pub intervals: Vec<IntervalDsl>,
    pub bass: Option<IntervalDsl>,
    pub duration_tweak: Option<DurationTweakDsl>,
}
impl ChordDsl {
    pub fn empty(empty: EmptyDsl) -> Self {
        Self {
            empty: Some(empty),
            root: None,
            intervals: vec![],
            bass: None,
            duration_tweak: None,
        }
    }
    pub fn chord(
        root: SyllableDsl,
        intervals: Vec<IntervalDsl>,
        bass: Option<IntervalDsl>,
        duration_tweak: Option<DurationTweakDsl>,
    ) -> Self {
        Self {
            empty: None,
            root: Some(root),
            intervals,
            bass,
            duration_tweak,
        }
    }
}

impl ChordDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, multied: bool, with_paren: bool) -> Self {
        if multied && !with_paren {
            throw!(Error::new(input.span(), "paren required in multied mode"));
        }
        if EmptyDsl::peek(input) {
            return ChordDsl::empty(input.parse()?);
        }
        let root = input.parse::<SyllableDsl>()?;
        input.parse::<Token![:]>()?;
        let mut intervals = Vec::new();
        while IntervalDsl::peek(input) {
            intervals.push(input.parse()?);
        }
        let base: Option<IntervalDsl> = if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        let duration_tweak = DurationTweakDsl::try_parse(input);
        ChordDsl::chord(root, intervals, base, duration_tweak)
    }
}

impl ToTokens for ChordDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ChordDsl {
            empty,
            root,
            intervals,
            bass,
            duration_tweak,
        } = self;
        let duration_quote = Context::duration_quote(duration_tweak);
        if empty.is_some() {
            tokens.extend(empty.as_ref().unwrap().quote(duration_quote));
        } else if let Some(root) = root {
            let intervals_quote: Vec<_> = intervals.iter().map(|x| quote! { #x }).collect();
            tokens.extend(match bass {
                Some(base) => quote! {
                    ProtoEntry::from(CoreEntry::from(
                        (Chord::from((#root, vec![
                            #(#intervals_quote),*
                        ], #base)), #duration_quote)
                    ))
                },
                None => quote! {
                    ProtoEntry::from(CoreEntry::from(
                        (Chord::from((#root, vec![
                            #(#intervals_quote),*
                        ])), #duration_quote)
                    ))
                },
            })
        }
    }
}

impl ChordDsl {
    pub fn to_proto(&self) -> ProtoEntry {
        let ChordDsl {
            empty,
            root,
            intervals,
            bass,
            duration_tweak,
        } = self;
        let duration = Context::tweaked_duration(duration_tweak);
        if empty.is_some() {
            empty.as_ref().unwrap().to_proto(duration)
        } else if let Some(root) = root {
            let intervals = intervals
                .iter()
                .map(|x| x.to_proto())
                .collect::<Vec<Interval>>();
            let bass = bass.as_ref().map(|x| x.to_proto().clone());
            ProtoEntry::from(CoreEntry::from((
                Chord::new(root.to_proto(), intervals.into(), bass),
                duration,
            )))
        } else {
            ProtoEntry::from(CoreEntry::from(duration))
        }
    }
}
