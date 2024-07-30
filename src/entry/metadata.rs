use super::file::File;
use std::fs::{read_link, FileType};
use std::os::unix::fs::FileTypeExt;
use users::{get_group_by_gid, get_user_by_uid};

const CYAN: &str = "\x1b[36m";
const BOLD_RED: &str = "\x1b[1;31m";
const BOLD_GREEN: &str = "\x1b[1;32m";
const BOLD_BLUE: &str = "\x1b[1;34m";
const BOLD_MAGENTA: &str = "\x1b[1;35m";
const BOLD_CYAN: &str = "\x1b[1;36m";
const BACK_RED: &str = "\x1b[41m";
const BACK_BLUE: &str = "\x1b[44m";
const BLACK_BACK_YELLOW: &str = "\x1b[30;43m";
const YELLOW_BACK_BLACK: &str = "\x1b[33;40m";
const BOLD_YELLOW_BACK_BLACK: &str = "\x1b[1;33;40m";
pub const RESET_COLOR: &str = "\x1b[0m";

pub fn get_owner(uid: u32, path_str: &str) -> String {
    let Some(user) = get_user_by_uid(uid) else {
        eprintln!("{}: Failed to get the owner!", path_str);
        return uid.to_string();
    };

    let Some(owner) = user.name().to_str() else {
        eprintln!("{}: Failed to get the owner!", path_str);
        return uid.to_string();
    };

    owner.to_string()
}

pub fn get_group(gid: u32, path_str: &str) -> String {
    let Some(user) = get_group_by_gid(gid) else {
        eprintln!("{}: Failed to get the group!", path_str);
        return gid.to_string();
    };

    let Some(owner) = user.name().to_str() else {
        eprintln!("{}: Failed to get the group!", path_str);
        return gid.to_string();
    };

    owner.to_string()
}

fn get_file_extension(path_str: &str) -> String {
    let extension: String =
        path_str.chars().rev().take_while(|&c| c != '.').collect();

    extension.chars().rev().collect()
}

fn is_exec(mode: u32) -> bool {
    mode & 0o100 != 0 || mode & 0o010 != 0 || mode & 0o001 != 0
}

pub fn get_color_escape(
    path_str: &str,
    file_type: FileType,
    mode: u32,
) -> &'static str {
    let mut ext_color = match get_file_extension(path_str).as_str() {
        "png" | "jpg" | "jpeg" | "webp" | "svg" | "gif" | "mp4" | "ppm"
        | "bmp" | "tiff" => BOLD_MAGENTA,
        "zip" | "tar" | "tgz" | "gz" | "rar" | "7z" | "jar" | "bz2" | "deb"
        | "war" => BOLD_RED,
        "mp3" | "ogg" | "wav" | "flac" | "aac" => CYAN,
        _ => "",
    };

    ext_color = match () {
        _ if mode & 0o4000 != 0 => BACK_RED,
        _ if mode & 0o2000 != 0 => BLACK_BACK_YELLOW,
        _ if is_exec(mode) => BOLD_GREEN,
        _ => ext_color,
    };

    match () {
        _ if file_type.is_dir() && mode & 0o1000 != 0 => BACK_BLUE,
        _ if file_type.is_dir() => BOLD_BLUE,
        _ if file_type.is_symlink() => BOLD_CYAN,
        _ if file_type.is_fifo() => YELLOW_BACK_BLACK,
        _ if file_type.is_socket() => BOLD_MAGENTA,
        _ if file_type.is_char_device() => BOLD_YELLOW_BACK_BLACK,
        _ if file_type.is_block_device() => BOLD_YELLOW_BACK_BLACK,
        _ => ext_color,
    }
}

fn get_symlink_dir_path(path_str: &str) -> String {
    let dir_path_str: String =
        path_str.chars().rev().skip_while(|&c| c != '/').collect();

    dir_path_str.chars().rev().collect()
}

pub fn get_symlink_target(
    sym_path_str: &str,
    is_symlink: bool,
) -> Option<Box<File>> {
    if !is_symlink {
        return None;
    }
    let Ok(path) = read_link(sym_path_str) else {
        eprintln!("{}: Failed to get the target of the symlink!", sym_path_str);
        return None;
    };
    let Some(path_str) = path.to_str() else {
        eprintln!("{}: Failed to get the target of the symlink!", sym_path_str);
        return None;
    };

    let mut dir_path_str = String::default();
    if !path_str.starts_with('/') {
        dir_path_str = get_symlink_dir_path(sym_path_str);
    }

    Some(Box::new(File::new(&(dir_path_str + path_str), path_str)))
}
