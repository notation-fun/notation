use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{LitInt, Token};

use crate::proto::mark::MarkDsl;

pub enum SliceBeginDsl {
    Mark(MarkDsl),
    Index(usize),
}
impl Parse for SliceBeginDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        if MarkDsl::peek(input) {
            Ok(Self::Mark(input.parse()?))
        } else {
            Ok(Self::Index(
                input.parse::<LitInt>()?.base10_parse::<usize>()?,
            ))
        }
    }
}
#[allow(dead_code)]
impl SliceBeginDsl {
    pub fn peek(input: ParseStream) -> bool {
        MarkDsl::peek(input) || input.peek(LitInt)
    }
}
pub enum SliceEndDsl {
    Mark(MarkDsl),
    Count(usize),
}
impl Parse for SliceEndDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        if MarkDsl::peek(input) {
            Ok(Self::Mark(input.parse()?))
        } else {
            Ok(Self::Count(
                input.parse::<LitInt>()?.base10_parse::<usize>()?,
            ))
        }
    }
}

pub struct SliceDsl {
    pub begin: SliceBeginDsl,
    pub end: SliceEndDsl,
    pub rounds: Option<Vec<usize>>,
}

impl SliceDsl {
    pub fn parse_without_brace(input: ParseStream) -> Result<Self> {
        let begin = input.parse()?;
        let end = input.parse()?;
        let mut rounds = None;
        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            let mut rounds_: Vec<usize> = Vec::new();
            while input.peek(LitInt) {
                rounds_.push(input.parse::<LitInt>()?.base10_parse::<usize>()?);
            }
            rounds = Some(rounds_);
        }
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        }
        Ok(SliceDsl { begin, end, rounds })
    }
    #[allow(dead_code)]
    pub fn peek(input: ParseStream) -> bool {
        SliceBeginDsl::peek(input)
    }
}

impl ToTokens for SliceBeginDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Mark(x) => quote! { SliceBegin::Mark(#x.to_owned()) },
            Self::Index(x) => quote! { SliceBegin::Index(#x) },
        });
    }
}
impl ToTokens for SliceEndDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Mark(x) => quote! { SliceEnd::Mark(#x.to_owned()) },
            Self::Count(x) => quote! { SliceEnd::Count(#x) },
        });
    }
}
impl ToTokens for SliceDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SliceDsl { begin, end, rounds } = self;
        let rounds_quote = match rounds {
            Some(rounds) => {
                quote! {
                    Some(vec![
                        #(#rounds),*
                    ])
                }
            }
            None => {
                quote! { None }
            }
        };
        tokens.extend(quote! {
            Slice::new(#begin, #end, #rounds_quote)
        });
    }
}
