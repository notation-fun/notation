use std::cmp::{max, min};

use fehler::throws;
use syn::parse::{Error, Parse, ParseStream};
use syn::{Token};
use notation_proto::prelude::Duration;

#[derive(Debug)]
pub struct DurationTweakDsl {
    pub half_num: i8,
    pub dotted: bool,
    pub triplet: bool,
}

impl Parse for DurationTweakDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let mut half_num: i8 = 0;
        while input.peek(Token![,]) || input.peek(Token![*]) {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                half_num += 1;
            } else {
                input.parse::<Token![*]>()?;
                half_num -= 1;
            }
        }
        half_num = min(max(half_num, -4), 4);
        let mut dotted = false;
        let mut triplet = false;
        if input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            dotted = true;
        } else if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            dotted = true;
            triplet = true;
        } else if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            triplet = true;
        }
        DurationTweakDsl { half_num, dotted, triplet }
    }
}

impl DurationTweakDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![,])
            || input.peek(Token![*])
            || input.peek(Token![.])
            || input.peek(Token![=])
            || input.peek(Token![-])
    }
    pub fn try_parse(input: ParseStream) -> Option<Self> {
        if Self::peek(input) {
            Self::parse(input).ok()
        } else {
            None
        }
    }
    pub fn tweak(&self, base: &Duration) -> Duration {
        if let Some(base_unit) = base.as_simple() {
            let mut unit = base_unit.clone();
            if self.half_num > 0 {
                for _ in 0..self.half_num {
                    unit = unit.halfed();
                }
            } else if self.half_num < 0 {
                for _ in 0..(-self.half_num) {
                    unit = unit.doubled();
                }
            }
            match (self.dotted, self.triplet) {
                (false, false) => Duration::Simple(unit),
                (true, false) => Duration::Dotted(unit),
                (false, true) => Duration::Triplet(unit),
                (true, true) => Duration::DottedTriplet(unit),
            }
        } else {
            println!("Can only tweak simple duration: {} - {:?}", base, self);
            return base.clone();
        }
    }
}