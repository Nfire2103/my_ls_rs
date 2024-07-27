use super::file::File;
use super::sort::sort_sub_entries;
use super::{display_error_at_open, Entry};
use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;

pub struct Folder {
    pub name: String,
    entries: Vec<Box<dyn Entry>>,
}

fn load_sub_entries(
    paths: Vec<PathBuf>,
    display_all: bool,
) -> Vec<Box<dyn Entry>> {
    let mut entries: Vec<Box<dyn Entry>> = Vec::new();

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

        if path.is_dir() {
            let folder_result = Folder::new(file_name, false, display_all)
                .map_err(|err| display_error_at_open(path_str, err));

            if let Ok(folder) = folder_result {
                entries.push(Box::new(folder));
            }
        } else {
            entries.push(Box::new(File::new(file_name.to_string())));
        }
    }

    if display_all {
        entries.push(Box::new(Folder::new(".", false, display_all).unwrap()));
        entries.push(Box::new(Folder::new("..", false, display_all).unwrap()));
    }

    sort_sub_entries(&mut entries);

    entries
}

impl Folder {
    pub fn new(
        path: &str,
        open_dir: bool,
        display_all: bool,
    ) -> Result<Self, Error> {
        let mut sub_paths = Vec::new();
        let mut entries = Vec::new();

        if open_dir {
            let mut read_dir = read_dir(path)?;

            while let Some(Ok(entry)) = read_dir.next() {
                sub_paths.push(entry.path());
            }

            entries = load_sub_entries(sub_paths, display_all);
        }

        Ok(Self {
            name: path.to_string(),
            entries,
        })
    }

    pub fn display_listed(&self) {
        for entry in &self.entries {
            entry.display();
        }

        if self.entries.len() > 0 {
            println!();
        }
    }
}

impl Entry for Folder {
    fn display(&self) {
        print!("{}  ", self.name);
    }

    fn get_name(&self) -> &String {
        return &self.name;
    }
}
