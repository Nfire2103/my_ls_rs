mod args;
mod entry;

use args::LISTING;
use entry::{load_entries, Entry};

macro_rules! printlnif {
    ($val:expr) => {
        if $val {
            println!();
        }
    };
}

fn main() {
    let parse_res = args::parse();

    let options = parse_res.options;
    let entries = load_entries(&options, parse_res.paths_str);
    let nbr_files = entries.files.len();
    let nbr_folders = entries.folders.len();
    let nbr_entries = nbr_files + nbr_folders;

    for file in entries.files {
        file.display(options[LISTING]);
    }
    printlnif!(nbr_files > 0 && !options[LISTING]);
    printlnif!(nbr_files > 0 && nbr_folders > 0);

    for (i, folder) in entries.folders.iter().enumerate() {
        if nbr_entries > 1 {
            println!("{}:", folder.get_name());
        }
        folder.display(options[LISTING]);
        printlnif!(i != nbr_folders - 1);
    }
}
