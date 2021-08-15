use fehler::{throw, throws};
use notation_proto::prelude::Interval;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitInt, Token};

mod kw {
    syn::custom_keyword!(t);
    syn::custom_keyword!(o);
}

pub struct IntervalDsl {
    pub interval: Interval,
}
impl IntervalDsl {
    pub fn new(interval: Interval) -> Self {
        Self { interval }
    }
}

impl Parse for IntervalDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        if input.peek(kw::t) {
            input.parse::<kw::t>()?;
            return IntervalDsl::new(Interval::Tritone);
        }
        let interval = match input.parse::<LitInt>()?.base10_parse::<u8>()? {
            1 => Interval::Unison,
            2 => {
                if input.peek(Token![-]) {
                    input.parse::<Token![-]>()?;
                    Interval::Minor2nd
                } else if input.peek(Token![+]) {
                    input.parse::<Token![+]>()?;
                    Interval::Augmented2nd
                } else {
                    Interval::Major2nd
                }
            }
            3 => {
                if input.peek(Token![-]) {
                    input.parse::<Token![-]>()?;
                    Interval::Minor3nd
                } else {
                    Interval::Major3nd
                }
            }
            4 => {
                if input.peek(kw::o) {
                    input.parse::<kw::o>()?;
                    Interval::Diminished4th
                } else {
                    Interval::Perfect4th
                }
            }
            5 => {
                if input.peek(Token![+]) {
                    input.parse::<Token![+]>()?;
                    Interval::Augmented5th
                } else {
                    Interval::Perfect5th
                }
            }
            6 => {
                if input.peek(Token![-]) {
                    input.parse::<Token![-]>()?;
                    Interval::Minor6th
                } else {
                    Interval::Major6th
                }
            }
            7 => {
                if input.peek(Token![-]) {
                    input.parse::<Token![-]>()?;
                    Interval::Minor7th
                } else {
                    Interval::Major7th
                }
            }
            8 => Interval::Perfect8ve,
            _ => throw!(Error::new(input.span(), "Invalid Interval")),
        };
        IntervalDsl::new(interval)
    }
}

impl IntervalDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(LitInt) || input.peek(kw::t)
    }
}

impl ToTokens for IntervalDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let interval_text = self.interval.to_text();
        tokens.extend(quote! {
            Interval::from_text(#interval_text)
        });
    }
}
