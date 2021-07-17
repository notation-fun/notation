use notation_proto::prelude::*;

#[derive(Debug)]
pub struct PlayToneEvent(pub Tone);

#[derive(Debug)]
pub struct StopToneEvent(pub Tone);
