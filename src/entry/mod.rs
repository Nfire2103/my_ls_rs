mod file;
mod folder;
mod sort;

use super::args::ALL;
use super::args::DIRECTORY;
use super::args::NBR_OPTIONS;
use super::args::RECURSIVE;
use file::File;
use folder::Folder;
use sort::sort_entries;
use std::env;
use std::io::Error;
use std::path::PathBuf;

pub struct Entries {
    pub files: Vec<File>,
    pub folders: Vec<Folder>,
}

pub trait Entry {
    fn display(&self, listing_format: bool);
    fn get_name(&self) -> &String;
}

fn display_no_such_file(path_str: &str) {
    println!(
        "{}: cannot access \'{}\': No such file or directory",
        env::args().nth(0).unwrap(),
        path_str
    );
}

fn display_error_at_open(path_str: &str, err: Error) {
    let mut err_kind = err.kind().to_string();

    println!(
        "{}: cannot open directory \'{}\': {}",
        env::args().nth(0).unwrap(),
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
            let folder_result =
                Folder::new(&path_str, options[ALL], options[RECURSIVE])
                    .map_err(|err| display_error_at_open(&path_str, err));

            if let Ok(folder) = folder_result {
                folders.push(folder);
            }
        } else {
            files.push(File::new(&path_str, &path_str));
        }
    }

    sort_entries(&mut files);
    sort_entries(&mut folders);

    Entries { files, folders }
}
