use super::Entry;
use chrono::{DateTime, Utc};
use std::fs::{read_link, symlink_metadata, FileType};
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use users::{get_group_by_gid, get_user_by_uid};

const RESET_COLOR: &str = "\x1B[0m";

#[derive(Default)]
pub struct File {
    mode: String,
    nlink: u64,
    owner: String,
    group: String,
    size: u64,
    pub blocks: u64,
    mtime: i64,
    mtime_str: String,
    color: &'static str,
    name: String,
    target: Option<Box<File>>,
}

fn get_char_type(file_type: FileType) -> char {
    match () {
        _ if file_type.is_dir() => 'd',
        _ if file_type.is_symlink() => 'l',
        _ if file_type.is_fifo() => 'p',
        _ if file_type.is_socket() => 's',
        _ if file_type.is_char_device() => 'c',
        _ if file_type.is_block_device() => 'b',
        _ => '-',
    }
}

fn format_mode(mode: u32, file_type: FileType) -> String {
    let mut mode_str = String::new();

    mode_str.push(get_char_type(file_type));

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

fn get_owner(uid: u32, path_str: &str) -> String {
    let Some(user) = get_user_by_uid(uid) else {
        println!("{}: Failed to get the owner!", path_str);
        return uid.to_string();
    };

    let Some(owner) = user.name().to_str() else {
        println!("{}: Failed to get the owner!", path_str);
        return uid.to_string();
    };

    owner.to_string()
}

fn get_group(gid: u32, path_str: &str) -> String {
    let Some(user) = get_group_by_gid(gid) else {
        println!("{}: Failed to get the group!", path_str);
        return gid.to_string();
    };

    let Some(owner) = user.name().to_str() else {
        println!("{}: Failed to get the group!", path_str);
        return gid.to_string();
    };

    owner.to_string()
}

fn format_mtime(mtime: i64, path_str: &str) -> String {
    let Some(datetime) = DateTime::<Utc>::from_timestamp(mtime, 0) else {
        println!("{}: Failed to get the timestamp!", path_str);
        return mtime.to_string();
    };

    datetime.format("%b %e %H:%M").to_string()
}

fn get_color_escape(file_type: FileType) -> &'static str {
    match () {
        _ if file_type.is_dir() => "\x1b[1;34m",
        _ if file_type.is_symlink() => "\x1b[1;36m",
        _ if file_type.is_fifo() => "\x1b[33;40m",
        _ if file_type.is_socket() => "\x1b[1;35m",
        _ if file_type.is_char_device() => "\x1b[1;33;40m",
        _ if file_type.is_block_device() => "\x1b[1;33;40m",
        _ => "",
    }
}

fn get_symlink_target(
    sym_path_str: &str,
    is_symlink: bool,
) -> Option<Box<File>> {
    if !is_symlink {
        return None;
    }
    let Ok(path) = read_link(sym_path_str) else {
        println!("{}: Failed to get the target of the symlink!", sym_path_str);
        return None;
    };
    let Some(path_str) = path.to_str() else {
        println!("{}: Failed to get the target of the symlink!", sym_path_str);
        return None;
    };

    Some(Box::new(File::new(path_str, path_str)))
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
            size: metada.size(),
            blocks: metada.blocks(),
            mtime: metada.mtime(),
            mtime_str: format_mtime(metada.mtime(), path_str),
            color: get_color_escape(metada.file_type()),
            name: file_name.to_string(),
            target: get_symlink_target(path_str, metada.is_symlink()),
        }
    }
}

impl Entry for File {
    fn display(&self, listing_format: bool) {
        if listing_format {
            print!(
                "{} {} {} {} {:>4} {} {}{}{}",
                self.mode,
                self.nlink,
                self.owner,
                self.group,
                self.size,
                self.mtime_str,
                self.color,
                self.name,
                RESET_COLOR,
            );

            if let Some(target) = &self.target {
                print!(" -> ");
                target.display(false);
            }

            println!();
        } else {
            print!("{}{}{}  ", self.color, self.name, RESET_COLOR);
        }
    }

    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_mtime(&self) -> i64 {
        return self.mtime;
    }
}
