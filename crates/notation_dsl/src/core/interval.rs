use fehler::{throw, throws};
use notation_proto::prelude::Interval;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitInt, Token};

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
        let interval = match input.parse::<LitInt>()?.base10_parse::<u8>()? {
            1 => Interval::Unison,
            2 | 9 => {
                if input.peek(Token![-]) {
                    input.parse::<Token![-]>()?;
                    Interval::Minor2nd
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
            4 | 11 => {
                if input.peek(Token![+]) {
                    input.parse::<Token![+]>()?;
                    Interval::Augmented4th
                } else {
                    Interval::Perfect4th
                }
            }
            5 => {
                if input.peek(Token![%]) {
                    input.parse::<Token![%]>()?;
                    Interval::Diminished5th
                } else if input.peek(Token![+]) {
                    input.parse::<Token![+]>()?;
                    Interval::Augmented5th
                } else {
                    Interval::Perfect5th
                }
            }
            6 | 13 => {
                if input.peek(Token![-]) {
                    input.parse::<Token![-]>()?;
                    Interval::Minor6th
                } else {
                    Interval::Major6th
                }
            }
            7 => {
                if input.peek(Token![%]) {
                    input.parse::<Token![%]>()?;
                    Interval::Diminished7th
                } else if input.peek(Token![-]) {
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
        input.peek(LitInt)
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

impl IntervalDsl {
    pub fn to_proto(&self) -> Interval {
        self.interval
    }
}
