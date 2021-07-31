use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{ParseStream, Result};
use syn::{Expr, LitStr};

use crate::proto::entry::EntryDsl;

pub struct LineDsl {
    pub key: LitStr,
    pub entries: Vec<EntryDsl>,
}

impl LineDsl {
    pub fn parse_without_brace(input: ParseStream) -> Result<Self> {
        let key = input.parse()?;
        let entries = EntryDsl::parse_vec(input)?;
        Ok(LineDsl { key, entries })
    }
}

impl ToTokens for LineDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let LineDsl { key, entries } = self;
        let entries_quote = EntryDsl::quote_vec(entries);
        tokens.extend(quote! {
            Line::new(#key.into(), #entries_quote)
        });
    }
}

pub enum LineDslOrExpr {
    Dsl(LineDsl),
    Expr(Expr),
}

impl LineDslOrExpr {
    pub fn parse_without_brace(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Dsl(input.parse()?))
        } else {
            Ok(Self::Expr(input.parse()?))
        }
    }
}

impl ToTokens for LineDslOrExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Dsl(x) => quote! { #x },
            Self::Expr(x) => quote! { #x },
        });
    }
}
