use notation_tab::prelude::*;

pub mod test;
pub mod scarborough_fair;
pub mod long_juan_feng;

pub fn main() {
    write_tab(&test::new_tab(), "../notation_viewer/assets/tabs/test.ron");
    write_tab(&scarborough_fair::new_tab(), "../notation_viewer/assets/tabs/scarborough_fair.ron");
    write_tab(&test::new_tab(), "../notation_viewer_cn/assets/tabs/test.ron");
    write_tab(&long_juan_feng::new_tab(), "../notation_viewer_cn/assets/tabs/long_juan_feng.ron");
}
