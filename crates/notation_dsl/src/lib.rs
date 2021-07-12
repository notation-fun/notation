#![feature(proc_macro_diagnostic)]

#[macro_use]
extern crate lazy_static;

use proc_macro::TokenStream;

use quote::ToTokens;
use syn::parse_macro_input;

mod bar;
mod context;
mod entry;
mod form;
mod fretted;
mod layer;
mod line;
mod section;
mod slice;
mod tab;
mod track;
mod util;

#[proc_macro]
pub fn entry(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as entry::EntryDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn line(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as line::LineDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn slice(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as slice::SliceDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn track(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as track::TrackDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn layer(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as layer::LayerDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn bar(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as bar::BarDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn section(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as section::SectionDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn form(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as form::FormDsl)
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn tab(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as tab::TabDsl)
        .into_token_stream()
        .into()
}
