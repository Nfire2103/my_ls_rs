mod args;
mod entry;

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

    let entries = load_entries(parse_res.options, parse_res.paths_str);
    let nbr_files = entries.files.len();
    let nbr_folders = entries.folders.len();
    let nbr_entries = nbr_files + nbr_folders;

    for file in entries.files {
        file.display();
    }
    printlnif!(nbr_files > 0);
    printlnif!(nbr_files > 0 && nbr_folders > 0);

    for (i, folder) in entries.folders.iter().enumerate() {
        if nbr_entries > 1 {
            println!("{}:", folder.get_name());
        }
        folder.display();
        printlnif!(i != nbr_folders - 1);
    }
}
