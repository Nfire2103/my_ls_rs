use super::file::File;
use super::sort::sort_entries;
use super::{display_error_at_open, Entries, Entry};
use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;

pub struct Folder {
    path_str: String,
    entries: Entries,
}

fn load_sub_entries(
    paths: Vec<PathBuf>,
    display_all: bool,
    open_sub_dirs: bool,
) -> Entries {
    let mut files = Vec::new();
    let mut folders = Vec::new();

    for path in paths {
        let Some(path_str) = path.to_str() else {
            continue;
        };
        let Some(os_name) = path.file_name() else {
            continue;
        };
        let Some(file_name) = os_name.to_str() else {
            continue;
        };
        if file_name.starts_with('.') && !display_all {
            continue;
        }

        if path.is_dir() && open_sub_dirs {
            let folder_result =
                Folder::new(path_str, display_all, open_sub_dirs)
                    .map_err(|err| display_error_at_open(path_str, err));

            if let Ok(folder) = folder_result {
                folders.push(folder);
            }
        }

        files.push(File::new(file_name));
    }

    if display_all {
        files.push(File::new("."));
        files.push(File::new(".."));
    }

    sort_entries(&mut files);
    sort_entries(&mut folders);

    Entries { files, folders }
}

impl Folder {
    pub fn new(
        path: &str,
        display_all: bool,
        open_sub_dirs: bool,
    ) -> Result<Self, Error> {
        let mut sub_paths = Vec::new();
        let mut read_dir = read_dir(path)?;

        while let Some(Ok(entry)) = read_dir.next() {
            sub_paths.push(entry.path());
        }

        Ok(Self {
            path_str: path.to_string(),
            entries: load_sub_entries(sub_paths, display_all, open_sub_dirs),
        })
    }
}

impl Entry for Folder {
    fn display(&self) {
        for entry in &self.entries.files {
            entry.display();
        }
        if self.entries.files.len() > 0 {
            println!();
        }

        for entry in &self.entries.folders {
            println!("\n{}:", entry.path_str);
            entry.display();
        }
    }

    fn get_name(&self) -> &String {
        return &self.path_str;
    }
}
