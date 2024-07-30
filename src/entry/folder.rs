use super::file::File;
use super::format::format_name;
use super::sort::sort_entries;
use super::{display_error_at_open, Entries, Entry};
use crate::args::{ALL, NBR_OPTIONS, RECURSIVE, REVERSE, TIME};
use std::fs::{read_dir, symlink_metadata};
use std::io::Error;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

#[derive(Default)]
pub struct Folder {
    path_str: String,
    mtime: i64,
    entries: Entries,
}

fn get_nbr_blks(files: &Vec<File>) -> u64 {
    files.iter().map(|f| f.blocks / 2).sum()
}

fn load_sub_entries(
    paths: Vec<PathBuf>,
    dir_path_str: &str,
    options: &[bool; NBR_OPTIONS],
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
        if file_name.starts_with('.') && !options[ALL] {
            continue;
        }

        if path.is_dir() && options[RECURSIVE] {
            let folder_result = Folder::new(path_str, options)
                .map_err(|err| display_error_at_open(path_str, err));

            if let Ok(folder) = folder_result {
                folders.push(folder);
            }
        }

        files.push(File::new(path_str, file_name));
    }

    if options[ALL] {
        files.push(File::new(&format!("{}{}", dir_path_str, "/."), "."));
        files.push(File::new(&format!("{}{}", dir_path_str, "/.."), ".."));
    }

    sort_entries(&mut files, options[TIME], options[REVERSE]);
    sort_entries(&mut folders, options[TIME], options[REVERSE]);

    Entries { files, folders }
}

impl Folder {
    pub fn new(
        path_str: &str,
        options: &[bool; NBR_OPTIONS],
    ) -> Result<Self, Error> {
        let mut sub_paths = Vec::new();
        let mut read_dir = read_dir(path_str)?;

        while let Some(Ok(dir_entry)) = read_dir.next() {
            sub_paths.push(dir_entry.path());
        }

        let Ok(metada) = symlink_metadata(path_str) else {
            println!("{}: Failed to load metadata!", path_str);
            return Ok(Self::default());
        };

        Ok(Self {
            path_str: path_str.to_string(),
            mtime: metada.mtime(),
            entries: load_sub_entries(sub_paths, path_str, options),
        })
    }
}

impl Entry for Folder {
    fn display(&self, is_listing: bool) {
        if is_listing {
            println!("total {}", get_nbr_blks(&self.entries.files));
        }
        for file in &self.entries.files {
            file.display(is_listing);
        }
        if self.entries.files.len() > 0 && !is_listing {
            println!();
        }

        for folder in &self.entries.folders {
            println!("\n{}:", format_name(&folder.path_str));
            folder.display(is_listing);
        }
    }

    fn get_name(&self) -> &String {
        return &self.path_str;
    }

    fn get_mtime(&self) -> i64 {
        return self.mtime;
    }
}
