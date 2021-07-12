use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::LitStr;

use crate::entry::EntryDsl;

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
