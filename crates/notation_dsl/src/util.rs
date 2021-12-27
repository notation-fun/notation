use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{braced, bracketed, parenthesized, token};

use crate::core::chord::ChordDsl;
use crate::core::tone::ToneDsl;
use crate::core::word::WordDsl;
use crate::fretted::pick::PickDsl;
use crate::proto::bar::BarDsl;
use crate::proto::entry::{EntryDsl, MultibleDsl};
use crate::proto::layer::LayerDsl;
use crate::proto::section::SectionDsl;
use crate::proto::slice::SliceDsl;
use crate::proto::track::TrackDsl;

macro_rules! impl_dsl {
    ($dsl_type:ident) => {
        impl Parse for $dsl_type {
            fn parse(input: ParseStream) -> Result<Self> {
                if input.peek(token::Brace) {
                    let content;
                    braced!(content in input);
                    Self::parse_without_brace(&content)
                } else {
                    Self::parse_without_brace(input)
                }
            }
        }
        #[allow(dead_code)]
        impl $dsl_type {
            #[throws(Error)]
            pub fn parse_vec(input: ParseStream) -> Vec<$dsl_type> {
                let mut result = vec![];
                if input.peek(token::Bracket) {
                    let content;
                    bracketed!(content in *input);
                    while !content.is_empty() {
                        result.push(content.parse()?);
                    }
                }
                result
            }
            pub fn quote_vec(v: &[$dsl_type]) -> TokenStream {
                let item_quotes: Vec<TokenStream> = v.iter().map(
                    |x| quote! { #x }
                ).collect();
                quote! {
                    vec![
                        #(#item_quotes),*
                    ]
                }
            }
        }
    }
}

impl_dsl!(EntryDsl);
impl_dsl!(SliceDsl);
impl_dsl!(TrackDsl);
impl_dsl!(LayerDsl);
impl_dsl!(BarDsl);
impl_dsl!(SectionDsl);

macro_rules! impl_multible_dsl {
    ($dsl_type:ident) => {
        impl Parse for $dsl_type {
            fn parse(input: ParseStream) -> Result<Self> {
                Self::parse_multible(input, false)
            }
        }
        #[allow(dead_code)]
        impl $dsl_type {
            fn parse_multible(input: ParseStream, multied: bool) -> Result<Self> {
                if input.peek(token::Paren) {
                    let content;
                    parenthesized!(content in input);
                    Self::parse_without_paren(&content, multied, true)
                } else {
                    Self::parse_without_paren(input, multied, false)
                }
            }
            #[throws(Error)]
            pub fn parse_vec(input: ParseStream) -> Vec<$dsl_type> {
                let mut result = vec![];
                if input.peek(token::Bracket) {
                    let content;
                    bracketed!(content in *input);
                    while !content.is_empty() {
                        result.push(Self::parse_multible(&content, true)?);
                    }
                }
                result
            }
            pub fn quote_multible(v: &MultibleDsl<$dsl_type>) -> TokenStream {
                let item_quotes: Vec<TokenStream> = v.items.iter().map(
                    |x| quote! { #x }
                ).collect();
                quote! {
                    vec![
                        #(#item_quotes),*
                    ]
                }
            }
        }
        impl Parse for MultibleDsl<$dsl_type> {
            fn parse(input: ParseStream) -> Result<Self> {
                let items =
                    if input.peek(token::Bracket) {
                        $dsl_type::parse_vec(input)?
                    } else {
                        vec![
                            $dsl_type::parse(input)?
                        ]
                    };
                Ok(Self { items } )
            }
        }
        impl ToTokens for MultibleDsl<$dsl_type> {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let item_quotes: Vec<TokenStream> = self.items.iter().map(
                    |x| quote! { #x }
                ).collect();
                tokens.extend(quote! {
                    #(#item_quotes),*
                });
            }
        }
    }
}

impl_multible_dsl!(ToneDsl);
impl_multible_dsl!(ChordDsl);
impl_multible_dsl!(WordDsl);
impl_multible_dsl!(PickDsl);
