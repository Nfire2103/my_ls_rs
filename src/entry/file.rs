use super::Entry;
use std::fs::metadata;
use std::os::unix::fs::MetadataExt;

pub struct File {
    mode: String,
    name: String,
}

impl File {
    pub fn new(path_str: &str, file_name: &str) -> Self {
        // TODO handle error correctly
        let metada = metadata(path_str).unwrap();

        Self {
            mode: Self::format_mode(metada.mode(), metada.is_dir()),
            name: file_name.to_string(),
        }
    }

    fn format_mode(mode: u32, is_dir: bool) -> String {
        let mut mode_str = String::new();

        mode_str.push(if is_dir { 'd' } else { '-' });

        mode_str.push(if mode & 0o400 != 0 { 'r' } else { '-' });
        mode_str.push(if mode & 0o200 != 0 { 'w' } else { '-' });
        mode_str.push(if mode & 0o100 != 0 { 'x' } else { '-' });

        mode_str.push(if mode & 0o040 != 0 { 'r' } else { '-' });
        mode_str.push(if mode & 0o020 != 0 { 'w' } else { '-' });
        mode_str.push(if mode & 0o010 != 0 { 'x' } else { '-' });

        mode_str.push(if mode & 0o004 != 0 { 'r' } else { '-' });
        mode_str.push(if mode & 0o002 != 0 { 'w' } else { '-' });
        mode_str.push(if mode & 0o001 != 0 { 'x' } else { '-' });

        mode_str
    }
}

impl Entry for File {
    fn display(&self, listing_format: bool) {
        if listing_format {
            println!("{} {}", self.mode, self.name);
        } else {
            print!("{}  ", self.name);
        }
    }

    fn get_name(&self) -> &String {
        return &self.name;
    }
}
