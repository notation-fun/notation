use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{Ident, LitStr};

use crate::bar::BarDsl;
use crate::entry::MultibleDsl;

pub struct SectionDsl {
    pub key: LitStr,
    pub kind: Ident,
    pub bars: MultibleDsl<BarDsl>,
}

impl SectionDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let key = input.parse()?;
        let kind = input.parse()?;
        let bars = input.parse()?;
        SectionDsl { key, kind, bars }
    }
}

impl ToTokens for SectionDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SectionDsl { key, kind, bars } = self;
        let kind_quote = kind.to_string();
        let bars_quote = BarDsl::quote_multible(bars);
        tokens.extend(quote! {
            Section::new(#key.into(), SectionKind::from_ident(#kind_quote), #bars_quote)
        });
    }
}
