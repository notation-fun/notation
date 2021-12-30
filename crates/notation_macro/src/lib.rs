#![feature(proc_macro_diagnostic)]

use notation_dsl::quote::ToTokens;
use proc_macro::TokenStream;

use notation_dsl::syn::parse_macro_input;

use notation_dsl::prelude::*;

#[proc_macro]
pub fn entry(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as EntryDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn slice(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as SliceDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn track(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as TrackDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn bar(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as BarDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn section(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as SectionDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn form(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as FormDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn tab(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as TabDsl)
        .into_token_stream()
        .into()
}
