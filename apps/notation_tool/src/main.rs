use ron::ser::{to_string_pretty, PrettyConfig};
use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "notation_tool")]
enum Args {
    WriteTab {
        /// which tab to write
        #[structopt(short, long, default_value = "test")]
        tab: String,

        /// Output file, stdout if not present
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
    },
    ListTabs {
        // The number of occurrences of the `v/verbose` flag
        /// Verbose mode (-v, -vv, -vvv, etc.)
        #[structopt(short, long, parse(from_occurrences))]
        verbose: u8,
    },
}

pub mod tab;

fn list_tabs(_verbose: u8) {
    let tabs = tab::get_tab_list();
    println!("\nTotal Tabs: {}\n", tabs.len());
    for tab in tabs {
        println!("    {}", tab.name);
    }
}

fn write_tab(tab: String, _output: Option<PathBuf>) {
    if let Some(tab) = tab::new_tab(tab.as_str()) {
        let pretty = PrettyConfig::new()
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true);
        let s = to_string_pretty(&tab, pretty).expect("Serialization failed");
        println!("{}", s);
    }
}

fn main() {
    match Args::from_args() {
        Args::WriteTab { tab, output } => write_tab(tab, output),
        Args::ListTabs { verbose } => list_tabs(verbose),
    };
}
