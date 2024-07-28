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

fn get_nbr_blks(files: &Vec<File>) -> u64 {
    files.iter().map(|f| f.blocks / 2).sum()
}

fn load_sub_entries(
    paths: Vec<PathBuf>,
    dir_path_str: &str,
    display_all: bool,
    open_sub_dirs: bool,
) -> Entries {
    let mut files = Vec::new();
    let mut folders = Vec::new();

    for path in paths {
        let Some(path_str) = path.to_str() else {
            continue;
        };
        let Some(os_file_name) = path.file_name() else {
            continue;
        };
        let Some(file_name) = os_file_name.to_str() else {
            continue;
        };
        if file_name.starts_with('.') && !display_all {
            continue;
        }

        // TODO maybe moove this if in display function
        if path.is_dir() && open_sub_dirs {
            let folder_result =
                Folder::new(path_str, display_all, open_sub_dirs)
                    .map_err(|err| display_error_at_open(path_str, err));

            if let Ok(folder) = folder_result {
                folders.push(folder);
            }
        }

        files.push(File::new(path_str, file_name));
    }

    if display_all {
        files.push(File::new(&format!("{}{}", dir_path_str, "/."), "."));
        files.push(File::new(&format!("{}{}", dir_path_str, "/.."), ".."));
    }

    sort_entries(&mut files);
    sort_entries(&mut folders);

    Entries { files, folders }
}

impl Folder {
    pub fn new(
        path_str: &str,
        display_all: bool,
        open_sub_dirs: bool,
    ) -> Result<Self, Error> {
        let mut sub_paths = Vec::new();
        let mut read_dir = read_dir(path_str)?;

        while let Some(Ok(dir_entry)) = read_dir.next() {
            sub_paths.push(dir_entry.path());
        }

        Ok(Self {
            path_str: path_str.to_string(),
            entries: load_sub_entries(
                sub_paths,
                path_str,
                display_all,
                open_sub_dirs,
            ),
        })
    }
}

impl Entry for Folder {
    fn display(&self, listing_format: bool) {
        if listing_format {
            println!("total {}", get_nbr_blks(&self.entries.files));
        }
        for file in &self.entries.files {
            file.display(listing_format);
        }
        if self.entries.files.len() > 0 && !listing_format {
            println!();
        }

        for folder in &self.entries.folders {
            println!("\n{}:", folder.path_str);
            folder.display(listing_format);
        }
    }

    fn get_name(&self) -> &String {
        return &self.path_str;
    }
}
