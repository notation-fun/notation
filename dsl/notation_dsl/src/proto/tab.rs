use fehler::throws;
use notation_proto::prelude::Tab;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitStr, Token};

use crate::proto::form::FormDsl;

use crate::proto::section::SectionDsl;
use crate::proto::track::TrackDsl;

use super::meta::MetaDsl;

pub struct TabDsl {
    pub uuid: String,
    pub meta: MetaDsl,
    pub tracks: Vec<TrackDsl>,
    pub sections: Vec<SectionDsl>,
    pub form: FormDsl,
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
        let uuid = input.parse::<LitStr>()?.value();

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
            uuid,
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
            uuid,
            meta,
            tracks,
            sections,
            form,
        } = self;
        let mata_quote = meta.to_token_stream();
        let tracks_quote = TrackDsl::quote_vec(tracks);
        let sections_quote = SectionDsl::quote_vec(sections);
        tokens.extend(quote! {
            Tab::new(
                #uuid,
                #mata_quote,
                #tracks_quote,
                #sections_quote,
                #form
            )
        });
    }
}

impl TabDsl {
    pub fn to_proto(&self) -> Tab {
        let meta = self.meta.to_proto();
        let tracks = self.tracks.iter().map(|x| x.to_proto()).collect();
        let sections = self.sections.iter().map(|x| x.to_proto()).collect();
        Tab::new(&self.uuid, meta, tracks, sections, self.form.to_proto())
    }
}
