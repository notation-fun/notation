use fehler::throws;

use notation_proto::prelude::{Signature, Unit};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Ident, LitInt};

pub struct SignatureDsl {
    pub bar_beats: u8,
    pub beat_unit: Unit,
}

impl Parse for SignatureDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let bar_beats = input.parse::<LitInt>()?.base10_parse::<u8>()?;
        let ident = input.parse::<Ident>()?;
        let beat_unit = Unit::from_ident(ident.to_string().as_str());
        SignatureDsl {
            bar_beats,
            beat_unit,
        }
    }
}

impl SignatureDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(LitInt)
    }
}

impl ToTokens for SignatureDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SignatureDsl {
            bar_beats,
            beat_unit,
        } = self;
        let beat_unit_ident = beat_unit.to_ident();
        tokens.extend(quote! {
            Signature::new(Unit::from_ident(#beat_unit_ident), #bar_beats)
        });
    }
}

impl SignatureDsl {
    pub fn to_proto(&self) -> Signature {
        Signature::new(self.beat_unit, self.bar_beats)
    }
}
