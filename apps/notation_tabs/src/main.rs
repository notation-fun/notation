use notation_tab::prelude::*;

pub mod long_juan_feng;
pub mod scarborough_fair;
pub mod test;

pub fn main() {
    write_tab(&test::new_tab(), "../notation_viewer/assets/tabs/test.ron");
    write_tab(
        &scarborough_fair::new_tab(),
        "../notation_viewer/assets/tabs/scarborough_fair.ron",
    );
    write_tab(
        &test::new_tab(),
        "../notation_viewer_cn/assets/tabs/test.ron",
    );
    write_tab(
        &long_juan_feng::new_tab(),
        "../notation_viewer_cn/assets/tabs/long_juan_feng.ron",
    );
    let result = parse_get_tab_file("src/test.rs");
    println!("{:?}", result);
}
