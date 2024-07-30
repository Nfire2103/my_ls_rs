use super::format::{format_mode, format_mtime, format_name};
use super::metadata::{
    get_color_escape, get_group, get_owner, get_symlink_target,
};
use super::Entry;
use std::fs::symlink_metadata;
use std::os::unix::fs::MetadataExt;

const RESET_COLOR: &str = "\x1B[0m";

#[derive(Default)]
pub struct File {
    mode: String,
    nlink: u64,
    owner: String,
    group: String,
    major: u64,
    minor: u64,
    size: u64,
    pub blocks: u64,
    mtime: i64,
    mtime_str: String,
    color: &'static str,
    name: String,
    target: Option<Box<File>>,
}

impl File {
    pub fn new(path_str: &str, file_name: &str) -> Self {
        let Ok(metada) = symlink_metadata(path_str) else {
            println!("{}: Failed to load metadata!", path_str);
            return Self::default();
        };

        Self {
            mode: format_mode(metada.mode(), metada.file_type()),
            nlink: metada.nlink(),
            owner: get_owner(metada.uid(), path_str),
            group: get_group(metada.gid(), path_str),
            major: major!(metada.rdev()),
            minor: minor!(metada.rdev()),
            size: metada.size(),
            blocks: metada.blocks(),
            mtime: metada.mtime(),
            mtime_str: format_mtime(metada.mtime(), path_str),
            color: get_color_escape(
                path_str,
                metada.file_type(),
                metada.mode(),
            ),
            name: file_name.to_string(),
            target: get_symlink_target(path_str, metada.is_symlink()),
        }
    }

    fn display_listing(&self) {
        print!(
            "{} {} {} {} ",
            self.mode, self.nlink, self.owner, self.group
        );

        if self.major != 0 {
            print!("{} {} ", self.major, self.minor);
        } else {
            print!("{:>4} ", self.size);
        }

        print!(
            "{} {}{}{}",
            self.mtime_str,
            self.color,
            format_name(&self.name),
            RESET_COLOR
        );

        if let Some(target) = &self.target {
            print!(" -> ");
            target.display(false);
        }

        println!();
    }

    fn display_simple(&self) {
        print!("{}{}{}  ", self.color, format_name(&self.name), RESET_COLOR);
    }
}

impl Entry for File {
    fn display(&self, is_listing: bool) {
        if is_listing {
            self.display_listing();
        } else {
            self.display_simple();
        }
    }

    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_mtime(&self) -> i64 {
        return self.mtime;
    }
}
