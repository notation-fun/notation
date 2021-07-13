use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{ParseStream, Result};
use syn::{LitInt, LitStr};

pub struct SliceDsl {
    pub line: LitStr,
    pub index: usize,
    pub count: usize,
}

impl SliceDsl {
    pub fn parse_without_brace(input: ParseStream) -> Result<Self> {
        let line = input.parse()?;
        let index = input.parse::<LitInt>()?.base10_parse::<usize>()?;
        let count = input.parse::<LitInt>()?.base10_parse::<usize>()?;
        Ok(SliceDsl { line, index, count })
    }
}

impl ToTokens for SliceDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SliceDsl { line, index, count } = self;
        tokens.extend(quote! {
            Slice::new(#line.into(), #index, #count)
        });
    }
}
