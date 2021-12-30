use crate::prelude::{GetTabDsl, TabDsl};
use anyhow::Error;
use quote::ToTokens;
use std::fs::File;
use std::io::Read;
use syn;

use notation_proto::prelude::*;

pub fn parse_get_tab(content: &str) -> Result<Tab, Error> {
    let ast = syn::parse_file(content)?;
    //println!("Last Item: {:#?}", ast.items.last().unwrap().as);
    let tokens = ast.items.last().unwrap().to_token_stream();
    //println!("{:#?}", tokens);
    let get_tab: GetTabDsl = syn::parse2(tokens)?;
    let tab = get_tab.tab;
    //println!("Tab: T:{}, S:{}", tab.tracks.len(), tab.sections.len());
    Ok(tab.to_proto())
}
pub fn parse_get_tab_file(path: &str) -> Result<Tab, Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("parse_get_tab_file: {} -> [{}]", path, content.len());
    parse_get_tab(&content)
}

pub fn parse_tab(content: &str) -> Result<Tab, Error> {
    let tab = syn::parse_str::<TabDsl>(content)?;
    //println!("Tab: T:{}, S:{}", tab.tracks.len(), tab.sections.len());
    Ok(tab.to_proto())
}
pub fn parse_tab_file(path: &str) -> Result<Tab, Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("parse_tab_file: {} -> [{}]", path, content.len());
    parse_tab(&content)
}
