use fehler::throws;

use notation_proto::prelude::{Key, Scale, TabMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::Ident;

use crate::context::Context;
use crate::core::signature::SignatureDsl;
use crate::core::tempo::TempoDsl;

pub struct MetaDsl {
    pub key: Ident,
    pub scale: Ident,
    pub signature: SignatureDsl,
    pub tempo: TempoDsl,
}

impl Parse for MetaDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let key = input.parse()?;
        let scale = input.parse()?;
        let signature = input.parse()?;
        let tempo = input.parse()?;
        MetaDsl {
            key,
            scale,
            signature,
            tempo,
        }
    }
}
impl ToTokens for MetaDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let MetaDsl {
            key,
            scale,
            signature,
            tempo,
        } = self;
        let key_ident = key.to_string();
        let scale_ident = scale.to_string();
        Context::set_key(Key::from_ident(key_ident.as_str()));
        Context::set_scale(Scale::from_ident(scale_ident.as_str()));
        tokens.extend(quote! {
            TabMeta::new(
                Key::from_ident(#key_ident),
                Scale::from_ident(#scale_ident),
                #signature,
                #tempo,
            )
        });
    }
}
impl MetaDsl {
    pub fn to_proto(&self) -> TabMeta {
        let key = Key::from_ident(self.key.to_string().as_str());
        let scale = Scale::from_ident(self.scale.to_string().as_str());
        Context::set_key(key);
        Context::set_scale(scale);
        TabMeta::new(key, scale, self.signature.to_proto(), self.tempo.to_proto())
    }
}
