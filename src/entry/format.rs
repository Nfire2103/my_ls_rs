use chrono::{DateTime, Utc};
use std::{fs::FileType, os::unix::fs::FileTypeExt};

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

pub fn format_mode(mode: u32, file_type: FileType) -> String {
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

pub fn format_mtime(mtime: i64, path_str: &str) -> String {
    let Some(datetime) = DateTime::<Utc>::from_timestamp(mtime, 0) else {
        println!("{}: Failed to get the timestamp!", path_str);
        return mtime.to_string();
    };

    datetime.format("%b %e %H:%M").to_string()
}

pub fn format_name(file_name: &str) -> String {
    if file_name.contains(' ') {
        format!("\'{}\'", file_name)
    } else {
        file_name.to_string()
    }
}
