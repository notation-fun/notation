use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Expr, Token};

use crate::proto::form::FormDsl;

use crate::proto::layer::LayerDsl;
use crate::proto::line::LineDslOrExpr;
use crate::proto::section::SectionDsl;
use crate::proto::track::TrackDsl;

pub struct TabDsl {
    meta: Expr,
    lines: Vec<LineDslOrExpr>,
    tracks: Vec<TrackDsl>,
    layers: Vec<LayerDsl>,
    sections: Vec<SectionDsl>,
    form: FormDsl,
}

mod kw {
    syn::custom_keyword!(meta);
    syn::custom_keyword!(lines);
    syn::custom_keyword!(tracks);
    syn::custom_keyword!(layers);
    syn::custom_keyword!(sections);
    syn::custom_keyword!(form);
}

impl Parse for TabDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        input.parse::<kw::meta>()?;
        input.parse::<Token![:]>()?;
        let meta = input.parse()?;

        input.parse::<kw::lines>()?;
        input.parse::<Token![:]>()?;
        let lines = LineDslOrExpr::parse_vec(input)?;

        input.parse::<kw::tracks>()?;
        input.parse::<Token![:]>()?;
        let tracks = TrackDsl::parse_vec(input)?;

        input.parse::<kw::layers>()?;
        input.parse::<Token![:]>()?;
        let layers = LayerDsl::parse_vec(input)?;

        input.parse::<kw::sections>()?;
        input.parse::<Token![:]>()?;
        let sections = SectionDsl::parse_vec(input)?;

        input.parse::<kw::form>()?;
        input.parse::<Token![:]>()?;
        let form = input.parse()?;

        TabDsl {
            meta,
            lines,
            tracks,
            layers,
            sections,
            form,
        }
    }
}

impl ToTokens for TabDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TabDsl {
            meta,
            lines,
            tracks,
            layers,
            sections,
            form,
        } = self;
        let lines_quote = LineDslOrExpr::quote_vec(lines);
        let tracks_quote = TrackDsl::quote_vec(tracks);
        let layers_quote = LayerDsl::quote_vec(layers);
        let sections_quote = SectionDsl::quote_vec(sections);
        tokens.extend(quote! {
            Tab::new(
                #meta,
                #lines_quote,
                #tracks_quote,
                #layers_quote,
                #sections_quote,
                #form
            )
        });
    }
}
