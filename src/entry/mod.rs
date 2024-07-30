#[macro_use]
mod macros;

mod file;
mod folder;
pub mod format;
pub mod metadata;
mod sort;

use crate::args::{DIRECTORY, NBR_OPTIONS, REVERSE, TIME};
use file::File;
use folder::Folder;
use sort::sort_entries;
use std::env;
use std::io::Error;
use std::path::PathBuf;

#[derive(Default)]
pub struct Entries {
    pub files: Vec<File>,
    pub folders: Vec<Folder>,
}

pub trait Entry {
    fn display(&self, is_listing: bool);
    fn get_name(&self) -> &String;
    fn get_mtime(&self) -> i64;
}

fn display_no_such_file(path_str: &str) {
    eprintln!(
        "{}: cannot access \'{}\': No such file or directory",
        env::args().nth(0).unwrap_or("my_ls_rs".to_string()),
        path_str
    );
}

fn display_error_at_open(path_str: &str, err: Error) {
    let mut err_kind = err.kind().to_string();

    eprintln!(
        "{}: cannot open directory \'{}\': {}",
        env::args().nth(0).unwrap_or("my_ls_rs".to_string()),
        path_str,
        format!("{}{}", err_kind.remove(0).to_uppercase(), err_kind)
    );
}

pub fn load_entries(
    options: &[bool; NBR_OPTIONS],
    paths_str: Vec<String>,
) -> Entries {
    let mut files = Vec::new();
    let mut folders = Vec::new();

    for path_str in paths_str {
        let path = PathBuf::from(&path_str);
        if !path.exists() {
            display_no_such_file(&path_str);
            continue;
        }

        if path.is_dir() && !options[DIRECTORY] {
            let folder_result = Folder::new(&path_str, options)
                .map_err(|err| display_error_at_open(&path_str, err));

            if let Ok(folder) = folder_result {
                folders.push(folder);
            }
        } else {
            files.push(File::new(&path_str, &path_str));
        }
    }

    sort_entries(&mut files, options[TIME], options[REVERSE]);
    sort_entries(&mut folders, options[TIME], options[REVERSE]);

    Entries { files, folders }
}
