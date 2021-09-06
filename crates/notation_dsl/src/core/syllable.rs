use fehler::{throw, throws};
use notation_proto::prelude::Syllable;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitInt, Token};

pub struct SyllableDsl {
    pub syllable: Syllable,
}
impl SyllableDsl {
    pub fn new(syllable: Syllable) -> Self {
        Self { syllable }
    }
    #[throws(Error)]
    fn parse_natural(input: ParseStream) -> Self {
        SyllableDsl::new(match input.parse::<LitInt>()?.base10_parse::<u8>()? {
            1 => Syllable::Do,
            2 => Syllable::Re,
            3 => Syllable::Mi,
            4 => Syllable::Fa,
            5 => Syllable::So,
            6 => Syllable::La,
            7 => Syllable::Si,
            _ => throw!(Error::new(input.span(), "Invalid Syllable")),
        })
    }
    #[throws(Error)]
    fn parse_sharp(input: ParseStream) -> Self {
        input.parse::<Token![#]>()?;
        SyllableDsl::new(match input.parse::<LitInt>()?.base10_parse::<u8>()? {
            1 => Syllable::Di,
            2 => Syllable::Ri,
            4 => Syllable::Fi,
            5 => Syllable::Si,
            6 => Syllable::Li,
            _ => throw!(Error::new(input.span(), "Invalid Syllable")),
        })
    }
    #[throws(Error)]
    fn parse_flat(input: ParseStream) -> Self {
        input.parse::<Token![%]>()?;
        SyllableDsl::new(match input.parse::<LitInt>()?.base10_parse::<u8>()? {
            2 => Syllable::Ra,
            3 => Syllable::Me,
            5 => Syllable::Se,
            6 => Syllable::Le,
            7 => Syllable::Te,
            _ => throw!(Error::new(input.span(), "Invalid Syllable")),
        })
    }
}

impl Parse for SyllableDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        if input.peek(Token![#]) {
            Self::parse_sharp(input)?
        } else if input.peek(Token![%]) {
            Self::parse_flat(input)?
        } else {
            Self::parse_natural(input)?
        }
    }
}

#[allow(dead_code)]
impl SyllableDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(LitInt) || input.peek(Token![#]) || input.peek(Token![%])
    }
}

impl ToTokens for SyllableDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let syllable_text = self.syllable.to_text();
        tokens.extend(quote! {
            Syllable::from_text(#syllable_text)
        });
    }
}
