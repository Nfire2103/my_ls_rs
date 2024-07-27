mod file;
mod folder;

use file::File;
use folder::Folder;
use std::env;
use std::io::Error;
use std::path::PathBuf;

pub trait Entry {
    fn display(&self);
}

fn display_no_such_file(path: &str) {
    println!(
        "{}: cannot access \'{}\': No such file or directory",
        env::args().nth(0).unwrap(),
        path
    );
}

fn display_error_at_open(path: &str, err: Error) {
    let mut err_type = err.kind().to_string();

    println!(
        "{}: cannot open directory \'{}\': {}",
        env::args().nth(0).unwrap(),
        path,
        format!("{}{}", err_type.remove(0).to_uppercase(), err_type)
    );
}

pub fn load_entries(paths: Vec<String>) -> (Vec<File>, Vec<Folder>) {
    let mut files = Vec::new();
    let mut folders = Vec::new();

    for path in paths {
        let entry = PathBuf::from(&path);
        if !entry.exists() {
            display_no_such_file(&path);
            continue;
        }

        if entry.is_dir() {
            let folder_result = Folder::new(&path, true)
                .map_err(|err| display_error_at_open(&path, err));

            if let Ok(folder) = folder_result {
                folders.push(folder);
            }
        } else {
            files.push(File::new(path));
        }
    }

    (files, folders)
}
