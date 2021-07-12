use notation_model::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct AddLineEvent(pub Arc<Line>);
