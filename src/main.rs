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
    let parse_result = args::parse();
    let entries = load_entries(&parse_result.0, parse_result.1);
    let nbr_files = entries.files.len();
    let nbr_folders = entries.folders.len();
    let nbr_entries = nbr_files + nbr_folders;

    for entry in entries.files {
        entry.display();
    }
    printlnif!(nbr_files > 0);
    printlnif!(nbr_files > 0 && nbr_folders > 0);

    for (i, entry) in entries.folders.iter().enumerate() {
        if nbr_entries > 1 {
            println!("{}:", entry.name);
        }
        entry.display_listed();
        printlnif!(i != nbr_folders - 1);
    }
}
