use fehler::{throw, throws};
use notation_proto::prelude::{CoreEntry, Duration};
use notation_proto::proto_entry::ProtoEntry;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Error, Parse, ParseStream};
use syn::Token;

pub enum EmptyDsl {
    Tie,
    Rest,
}

impl Parse for EmptyDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Self::Tie
        } else if input.peek(Token![_]) {
            input.parse::<Token![_]>()?;
            Self::Rest
        } else {
            throw!(Error::new(input.span(), "Invalid EmptyDsl"))
        }
    }
}

impl EmptyDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![@]) || input.peek(Token![_])
    }
    pub fn quote(&self, duration_quote: TokenStream) -> TokenStream {
        match self {
            Self::Tie => quote! {
                ProtoEntry::from(CoreEntry::from(()))
            },
            Self::Rest => quote! {
                ProtoEntry::from(CoreEntry::from(#duration_quote))
            },
        }
    }
}

impl EmptyDsl {
    pub fn to_proto(&self, duration: Duration) -> ProtoEntry {
        match self {
            Self::Tie => ProtoEntry::from(CoreEntry::from(())),
            Self::Rest => ProtoEntry::from(CoreEntry::from(duration)),
        }
    }
}
