#![feature(proc_macro_diagnostic)]

#[macro_use]
extern crate lazy_static;

use proc_macro::TokenStream;

use quote::ToTokens;
use syn::parse_macro_input;

mod context;
mod core;
mod fretted;
mod proto;
mod util;

#[proc_macro]
pub fn entry(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as proto::entry::EntryDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn slice(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as proto::slice::SliceDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn track(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as proto::track::TrackDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn bar(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as proto::bar::BarDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn section(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as proto::section::SectionDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn form(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as proto::form::FormDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn tab(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as proto::tab::TabDsl)
        .into_token_stream()
        .into()
}
