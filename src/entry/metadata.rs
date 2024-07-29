use super::file::File;
use std::fs::{read_link, FileType};
use std::os::unix::fs::FileTypeExt;
use users::{get_group_by_gid, get_user_by_uid};

pub fn get_owner(uid: u32, path_str: &str) -> String {
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

pub fn get_group(gid: u32, path_str: &str) -> String {
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
        | "bmp" | "tiff" => "\x1b[1;31m",
        "zip" | "tar" | "tgz" | "gz" | "rar" | "7z" | "jar" | "bz2" | "deb"
        | "war" => "\x1b[1;35m",
        "mp3" | "ogg" | "wav" | "flac" | "aac" => "\x1b[36m",
        _ => "",
    };

    ext_color = match () {
        _ if mode & 0o4000 != 0 => "\x1b[41m",
        _ if mode & 0o2000 != 0 => "\x1b[30;43m",
        _ if is_exec(mode) => "\x1b[1;32m",
        _ => ext_color,
    };

    match () {
        _ if file_type.is_dir() => "\x1b[1;34m",
        _ if file_type.is_symlink() => "\x1b[1;36m",
        _ if file_type.is_fifo() => "\x1b[33;40m",
        _ if file_type.is_socket() => "\x1b[1;35m",
        _ if file_type.is_char_device() => "\x1b[1;33;40m",
        _ if file_type.is_block_device() => "\x1b[1;33;40m",
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
        println!("{}: Failed to get the target of the symlink!", sym_path_str);
        return None;
    };
    let Some(path_str) = path.to_str() else {
        println!("{}: Failed to get the target of the symlink!", sym_path_str);
        return None;
    };

    let mut dir_path_str: String = String::default();
    if !path_str.starts_with('/') {
        dir_path_str = get_symlink_dir_path(sym_path_str);
    }

    Some(Box::new(File::new(&(dir_path_str + path_str), path_str)))
}
