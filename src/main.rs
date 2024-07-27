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
    let entries = load_entries(parse_result.1);
    let nbr_files = entries.0.len();
    let nbr_folders = entries.1.len();
    let nbr_entries = nbr_files + nbr_folders;

    for entry in entries.0 {
        entry.display();
    }
    printlnif!(nbr_files > 0);
    printlnif!(nbr_files > 0 && nbr_folders > 0);

    for (i, entry) in entries.1.iter().enumerate() {
        if nbr_entries > 1 {
            println!("{}:", entry.name);
        }
        entry.display_listed();
        printlnif!(i != nbr_folders - 1);
    }
}
