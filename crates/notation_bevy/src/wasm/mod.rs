extern crate web_sys;

#[macro_use]
pub mod web_utils {
    // A macro to provide `println!(..)`-style syntax for `console.log` logging.
    macro_rules! web_log {
        ( $( $t:tt )* ) => {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
}

pub mod bevy_web_fullscreen;
