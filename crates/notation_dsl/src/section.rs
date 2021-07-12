use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Ident, LitStr};

use crate::bar::BarDsl;

pub struct SectionDsl {
    pub key: LitStr,
    pub kind: Ident,
    pub bars: Vec<BarDsl>,
}

impl SectionDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let key = input.parse()?;
        let kind = input.parse()?;
        let bars = BarDsl::parse_vec(input)?;
        SectionDsl { key, kind, bars }
    }
}

impl ToTokens for SectionDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SectionDsl { key, kind, bars } = self;
        let kind_quote = kind.to_string();
        let bars_quote = BarDsl::quote_vec(bars);
        tokens.extend(quote! {
            Section::new(#key.into(), SectionKind::from_ident(#kind_quote), #bars_quote)
        });
    }
}
