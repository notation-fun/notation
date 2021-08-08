use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Expr, Token};

use crate::proto::form::FormDsl;

use crate::proto::section::SectionDsl;
use crate::proto::track::TrackDsl;

pub struct TabDsl {
    meta: Expr,
    tracks: Vec<TrackDsl>,
    sections: Vec<SectionDsl>,
    form: FormDsl,
}

mod kw {
    syn::custom_keyword!(Meta);
    syn::custom_keyword!(Tracks);
    syn::custom_keyword!(Sections);
    syn::custom_keyword!(Form);
}

impl Parse for TabDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        input.parse::<kw::Meta>()?;
        input.parse::<Token![:]>()?;
        let meta = input.parse()?;

        input.parse::<kw::Tracks>()?;
        input.parse::<Token![:]>()?;
        let tracks = TrackDsl::parse_vec(input)?;

        input.parse::<kw::Sections>()?;
        input.parse::<Token![:]>()?;
        let sections = SectionDsl::parse_vec(input)?;

        input.parse::<kw::Form>()?;
        input.parse::<Token![:]>()?;
        let form = input.parse()?;

        TabDsl {
            meta,
            tracks,
            sections,
            form,
        }
    }
}

impl ToTokens for TabDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TabDsl {
            meta,
            tracks,
            sections,
            form,
        } = self;
        let tracks_quote = TrackDsl::quote_vec(tracks);
        let sections_quote = SectionDsl::quote_vec(sections);
        tokens.extend(quote! {
            Tab::new(
                #meta,
                #tracks_quote,
                #sections_quote,
                #form
            )
        });
    }
}
