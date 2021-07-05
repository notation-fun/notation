use notation_proto::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct AddLineEvent(pub Arc<Line>);
