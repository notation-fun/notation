use fehler::throws;

use notation_proto::prelude::{Section, SectionKind};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::Ident;

use crate::proto::bar::BarDsl;

use super::id::IdDsl;

pub struct SectionDsl {
    pub id: IdDsl,
    pub kind: Ident,
    pub bars: Vec<BarDsl>,
}

impl SectionDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let id = input.parse()?;
        let kind = input.parse()?;
        let bars = BarDsl::parse_vec(input)?;
        SectionDsl { id, kind, bars }
    }
}

impl ToTokens for SectionDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SectionDsl { id, kind, bars } = self;
        let kind_quote = kind.to_string();
        let bars_quote = BarDsl::quote_vec(bars);
        tokens.extend(quote! {
            Section::new(#id.into(), SectionKind::from_ident(#kind_quote), #bars_quote)
        });
    }
}

impl SectionDsl {
    pub fn to_proto(&self) -> Section {
        let bars = self.bars.iter().map(|x| x.to_proto()).collect();
        Section::new(
            self.id.id.clone(),
            SectionKind::from_ident(self.kind.to_string().as_str()),
            bars,
        )
    }
}
