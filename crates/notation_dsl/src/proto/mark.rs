use fehler::{throw, throws};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitStr, Token};

pub struct MarkDsl {
    pub mark: String,
}

impl Parse for MarkDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mark =
            if input.peek(Token![|]) {
                input.parse::<Token![|]>()?;
                "|".to_owned()
            } else if input.peek(LitStr) {
                input.parse::<LitStr>()?.value()
            } else {
                throw!(Error::new(input.span(), "Invalid Mark"))
            };
        Self { mark }
    }
}

impl MarkDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![|])
            || input.peek(LitStr)
    }
}

impl ToTokens for MarkDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let MarkDsl { mark } = self;
        tokens.extend(quote! {
            #mark
        });
    }
}
