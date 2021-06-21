use bevy::prelude::*;

use notation_core::prelude::Duration;

#[derive(Bundle)]
pub struct EntryBundle {
    buration: Duration,
}
