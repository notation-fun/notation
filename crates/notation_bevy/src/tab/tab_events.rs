use notation_proto::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct AddTabEvent(pub Arc<Tab>);
