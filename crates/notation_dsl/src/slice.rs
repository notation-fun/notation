use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{LitInt, LitStr};

pub enum SliceBeginDsl {
    Mark(LitStr),
    Index(usize),
}
impl Parse for SliceBeginDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Mark(input.parse()?))
        } else {
            Ok(Self::Index(
                input.parse::<LitInt>()?.base10_parse::<usize>()?,
            ))
        }
    }
}

pub enum SliceEndDsl {
    Mark(LitStr),
    Count(usize),
}
impl Parse for SliceEndDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Mark(input.parse()?))
        } else {
            Ok(Self::Count(
                input.parse::<LitInt>()?.base10_parse::<usize>()?,
            ))
        }
    }
}

pub struct SliceDsl {
    pub line: LitStr,
    pub begin: SliceBeginDsl,
    pub end: SliceEndDsl,
}

impl SliceDsl {
    pub fn parse_without_brace(input: ParseStream) -> Result<Self> {
        let line = input.parse()?;
        let begin = input.parse()?;
        let end = input.parse()?;
        Ok(SliceDsl { line, begin, end })
    }
}

impl ToTokens for SliceBeginDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Mark(x) => quote! { SliceBegin::Mark(#x.to_string()) },
            Self::Index(x) => quote! { SliceBegin::Index(#x) },
        });
    }
}
impl ToTokens for SliceEndDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Mark(x) => quote! { SliceEnd::Mark(#x.to_string()) },
            Self::Count(x) => quote! { SliceEnd::Count(#x) },
        });
    }
}
impl ToTokens for SliceDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SliceDsl { line, begin, end } = self;
        tokens.extend(quote! {
            Slice::new(#line.into(), #begin, #end)
        });
    }
}
