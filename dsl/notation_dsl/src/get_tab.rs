use fehler::throws;
use syn::ext::IdentExt;
use syn::parse::{Error, Parse, ParseStream};
use syn::{braced, parenthesized, Ident, Token};

use crate::prelude::TabDsl;

pub struct GetTabDsl {
    pub tab: TabDsl,
}

impl Parse for GetTabDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let _ = input.call(Ident::parse_any)?; //pub
        let _ = input.call(Ident::parse_any)?; //pub
        let _ = input.parse::<Ident>()?; //new_tab
        let _params_content; // ()
        parenthesized!(_params_content in input);
        let _ = input.parse::<Token![-]>()?;
        let _ = input.parse::<Token![>]>()?;
        let _ = input.parse::<Ident>()?; //Tab
        let fn_content;
        braced!(fn_content in input);
        let _ = fn_content.parse::<Ident>(); //tab
        let _ = fn_content.parse::<Token![!]>();
        let tab_content;
        braced!(tab_content in fn_content);
        let tab = tab_content.parse()?;
        GetTabDsl { tab }
    }
}
