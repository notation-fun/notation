use fehler::{throw, throws};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{LitInt, Token};

use crate::context::Context;

use super::duration::DurationTweakDsl;
use super::empty::EmptyDsl;
use super::interval::IntervalDsl;
use super::syllable::SyllableDsl;

pub struct ChordDsl {
    pub empty: Option<EmptyDsl>,
    pub root: Option<SyllableDsl>,
    pub intervals: Vec<IntervalDsl>,
    pub base: Option<usize>,
    pub duration_tweak: Option<DurationTweakDsl>,
}
impl ChordDsl {
    pub fn empty(empty: EmptyDsl) -> Self {
        Self {
            empty: Some(empty),
            root: None,
            intervals: vec![],
            base: None,
            duration_tweak: None,
        }
    }
    pub fn chord(
        root: SyllableDsl,
        intervals: Vec<IntervalDsl>,
        base: Option<usize>,
        duration_tweak: Option<DurationTweakDsl>,
    ) -> Self {
        Self {
            empty: None,
            root: Some(root),
            intervals,
            base,
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
        let base = if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            Some(input.parse::<LitInt>()?.base10_parse::<usize>()?)
        } else {
            None
        };
        if let Some(base) = base {
            if base >= intervals.len() {
                throw!(Error::new(input.span(), "Base Slash Out of Range"));
            }
        }
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
            base,
            duration_tweak,
        } = self;
        let duration_quote = Context::duration_quote(duration_tweak);
        if empty.is_some() {
            tokens.extend(empty.as_ref().unwrap().quote(duration_quote));
        } else if let Some(root) = root {
            let intervals_quote: Vec<_> = intervals.iter().map(|x| quote! { #x }).collect();
            tokens.extend(match base {
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
