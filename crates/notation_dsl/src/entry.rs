use fehler::{throw, throws};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{braced, bracketed, token, Ident, Token};

use crate::context::ContextDsl;
use crate::fretted::fretboard::FretboardDsl;
use crate::fretted::pick::PickDsl;
use crate::fretted::shape::ShapeDsl;

pub enum EntryDsl {
    Pick(PickDsl),
    Shape(ShapeDsl),
    Fretboard(FretboardDsl),
    Context(ContextDsl),
}

impl EntryDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        if input.peek(Token![!]) {
            input.parse::<Token![!]>()?;
            Self::Context(input.parse()?)
        } else {
            match input.parse::<Ident>()?.to_string().as_str() {
                "Pick" => Self::Pick(input.parse()?),
                "Shape" => Self::Shape(input.parse()?),
                "Fretboard" => Self::Fretboard(input.parse()?),
                _ => throw!(Error::new(input.span(), "Invalid Entry")),
            }
        }
    }
}

impl ToTokens for EntryDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Pick(x) => quote! { #x },
            Self::Shape(x) => quote! { #x },
            Self::Fretboard(x) => quote! { #x },
            Self::Context(x) => quote! { #x },
        });
    }
}
