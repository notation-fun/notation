use notation_tab::prelude::*;

pub mod scarborough_fair;

pub fn main() {
    write_tab(&scarborough_fair::new_tab(), "../notation_viewer/assets/tabs/scarborough_fair.ron");
}
