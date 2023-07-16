use fehler::throws;

use notation_proto::prelude::Tempo;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Ident, LitInt};

pub struct TempoDsl {
    pub tempo: Tempo,
}

impl Parse for TempoDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let tempo = if input.peek(LitInt) {
            let bpm = input.parse::<LitInt>()?.base10_parse::<u16>()?;
            Tempo::Bpm(bpm)
        } else {
            let ident = input.parse::<Ident>()?;
            Tempo::from_ident(ident.to_string().as_str())
        };
        TempoDsl { tempo }
    }
}

impl TempoDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(LitInt) || input.peek(Ident)
    }
}

impl ToTokens for TempoDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TempoDsl { tempo } = self;
        let tempo_ident = tempo.to_ident();
        tokens.extend(quote! {
            Tempo::from_ident(#tempo_ident)
        });
    }
}

impl TempoDsl {
    pub fn to_proto(&self) -> Tempo {
        self.tempo
    }
}
