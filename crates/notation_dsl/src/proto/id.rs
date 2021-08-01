use fehler::{throw, throws};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Ident, LitStr};

pub struct IdDsl {
    pub id: String,
}

impl Parse for IdDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let key =
            if input.peek(Ident) {
                input.parse::<Ident>()?.to_string()
            } else if input.peek(LitStr) {
                input.parse::<LitStr>()?.value()
            } else {
                throw!(Error::new(input.span(), "Invalid Key"))
            };
        Self { id: key }
    }
}

impl IdDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Ident)
            || input.peek(LitStr)
    }
}

impl ToTokens for IdDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdDsl { id } = self;
        tokens.extend(quote! {
            #id
        });
    }
}
