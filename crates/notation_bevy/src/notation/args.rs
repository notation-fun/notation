#[cfg(feature = "native")]
use clap::Parser;

use bevy::prelude::*;

#[derive(Debug, Resource)]
#[cfg_attr(feature = "native", derive(Parser))]
#[cfg_attr(feature = "native", clap(author, version, about, long_about = None))]
pub struct NotationArgs {
    #[cfg_attr(feature = "native", clap(short, long, default_value = "en-US"))]
    pub lang: String,

    #[cfg_attr(feature = "native", clap(short, long))]
    pub tab: Vec<String>,
}

impl NotationArgs {
    pub fn parse_args() -> Self {
        #[cfg(feature = "native")]
        return Self::parse_native();
        #[cfg(target_arch = "wasm32")]
        return Self::parse_wasm();
        Self {
            lang: "en-US".to_owned(),
            tab: vec![ "tabs/test.ron".to_owned() ]
        }
    }
    #[cfg(feature = "native")]
    pub fn parse_native() -> Self {
        use crate::settings::notation_settings::NotationSettings;

        let mut args = Self::parse();
        println!("NotationArgs::parse_native() -> {:#?}", args);
        if args.tab.len() == 0 {
            args.tab.push("tabs/test.ron".to_owned());
            args.tab.push("tabs/scarborough_fair.ron".to_owned());
            if args.lang == NotationSettings::ZH_CN.to_string() {
                args.tab.push("tabs/zh-CN/long_juan_feng.ron".to_owned());
            }
        }
        args
    }
    #[cfg(target_arch = "wasm32")]
    pub fn parse_wasm() -> Self {
        use crate::settings::notation_settings::NotationSettings;

        let mut lang = NotationSettings::EN_US.to_string();
        let mut tab = vec![];
        match web_sys::window().ok_or("No_Window".to_owned())
            .and_then(|x| x.document().ok_or("No_Document".to_owned()))
            .and_then(|x| x.location().ok_or("No_Location".to_owned()))
            .and_then(|x| x.href().map_err(|e| format!("href:{:?}", e)))
            .and_then(|x| web_sys::Url::new(x.as_str()).map_err(|e| format!("url:{:?}", e)))
            .map(|x| x.search_params()) {
            Ok(params) => {
                if let Some(v) = params.get("lang") {
                    lang = NotationSettings::parse_lang(&v).to_string();
                }
                if let Some(v) = params.get("tab") {
                    tab.push(v.clone());
                }
            },
            Err(err) => {
                println!("NotationArgs::parse_wasm() Failed: {}", err);
            }
        }
        if tab.len() == 0 {
            tab.push("tabs/test.ron".to_owned());
        }
        Self {
            lang,
            tab,
        }
    }
}