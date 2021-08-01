use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{Ident, LitStr};

use crate::proto::entry::EntryDsl;

use super::id::IdDsl;

pub struct TrackDsl {
    pub id: IdDsl,
    pub kind: Ident,
    pub entries: Vec<EntryDsl>,
}

impl TrackDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let id = input.parse()?;
        let kind = input.parse()?;
        let entries = EntryDsl::parse_vec(input)?;
        TrackDsl { id, kind, entries }
    }
}

impl ToTokens for TrackDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TrackDsl { id, kind, entries } = self;
        let kind_quote = kind.to_string();
        let entries_quote = EntryDsl::quote_vec(entries);
        tokens.extend(quote! {
            Track::new(#id.into(), TrackKind::from_ident(#kind_quote), #entries_quote)
        });
    }
}
