use std::env;
use std::process;

pub const ALL: usize = 0;
pub const LISTING: usize = 1;
pub const RECURSIVE: usize = 2;
pub const DIRECTORY: usize = 3;
pub const REVERSE: usize = 4;
pub const TIME: usize = 5;
pub const NBR_OPTIONS: usize = 6;

pub struct ParseRes {
    pub options: [bool; NBR_OPTIONS],
    pub paths_str: Vec<String>,
}

fn display_help() {
    println!(
        "Usage: {} [OPTION]... [FILE]...",
        env::args().nth(0).unwrap_or("my_ls_rs".to_string())
    );
    println!(
        "List information about the FILEs (the current directory by default)."
    );
    process::exit(0);
}

fn exit_bad_big_option(option_str: &str) {
    eprintln!(
        "{}: unrecognized option \'{}\'",
        env::args().nth(0).unwrap_or("my_ls_rs".to_string()),
        option_str
    );
    eprintln!(
        "Try \'{} --help\' for more information.",
        env::args().nth(0).unwrap_or("my_ls_rs".to_string())
    );
    process::exit(2);
}

fn retrieve_big_option(arg: &str, _: &mut [bool; NBR_OPTIONS]) {
    match arg {
        "--help" => display_help(),
        _ => exit_bad_big_option(arg),
    }
}

fn exit_bad_small_option(option_c: char) {
    eprintln!(
        "{}: option requires an argument -- \'{}\'",
        env::args().nth(0).unwrap_or("my_ls_rs".to_string()),
        option_c
    );
    eprintln!(
        "Try \'{} --help\' for more information.",
        env::args().nth(0).unwrap_or("my_ls_rs".to_string())
    );
    process::exit(2);
}

fn retrieve_small_options(arg: &str, options: &mut [bool; NBR_OPTIONS]) {
    for option_c in arg.chars().skip(1) {
        match option_c {
            'a' => options[ALL] = true,
            'l' => options[LISTING] = true,
            'R' => options[RECURSIVE] = true,
            'd' => options[DIRECTORY] = true,
            'r' => options[REVERSE] = true,
            't' => options[TIME] = true,
            _ => exit_bad_small_option(option_c),
        }
    }
}

pub fn parse() -> ParseRes {
    let mut options = [false; NBR_OPTIONS];
    let mut paths_str = Vec::new();

    for arg in env::args().skip(1) {
        let (Some(first_char), Some(second_char)) =
            (arg.chars().nth(0), arg.chars().nth(1))
        else {
            paths_str.push(arg);
            continue;
        };

        if first_char == '-' && second_char == '-' {
            retrieve_big_option(&arg, &mut options);
        } else if first_char == '-' {
            retrieve_small_options(&arg, &mut options);
        } else {
            paths_str.push(arg);
        }
    }

    if paths_str.is_empty() {
        paths_str.push(".".to_string());
    }

    ParseRes { options, paths_str }
}
