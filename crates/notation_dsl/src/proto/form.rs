use fehler::throws;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};

use super::id::IdDsl;

pub struct FormDsl {
    pub sections: Vec<IdDsl>,
}

impl Parse for FormDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mut sections = Vec::new();
        while IdDsl::peek(input) {
            sections.push(input.parse()?);
        }
        FormDsl { sections }
    }
}
impl ToTokens for FormDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FormDsl { sections } = self;
        tokens.extend(quote! {
            Form::from(vec![
                #(#sections),*
            ])
        });
    }
}
