use std::env;
use std::process;
use std::str::Chars;

pub const ALL: usize = 0;
pub const LISTING: usize = 1;
pub const RECURSIVE: usize = 2;
pub const DIRECTORY: usize = 3;
pub const REVERSE: usize = 4;
pub const TIME: usize = 5;
pub const NBR_OPTIONS: usize = 6;

fn display_help() {
    println!(
        "Usage: {} [OPTION]... [FILE]...",
        env::args().nth(0).unwrap()
    );
    println!(
        "List information about the FILEs (the current directory by default)."
    );
    process::exit(0);
}

fn exit_bad_big_option(option: &str) {
    println!(
        "{}: unrecognized option \'{}\'",
        env::args().nth(0).unwrap(),
        option
    );
    println!(
        "Try \'{} --help\' for more information.",
        env::args().nth(0).unwrap()
    );
    process::exit(2);
}

fn retrieve_big_option(arg: &str, _: &mut [bool; NBR_OPTIONS]) {
    match arg {
        "--help" => display_help(),
        _ => exit_bad_big_option(arg),
    }
}

fn exit_bad_small_option(option: char) {
    println!(
        "{}: option requires an argument -- \'{}\'",
        env::args().nth(0).unwrap(),
        option
    );
    println!(
        "Try \'{} --help\' for more information.",
        env::args().nth(0).unwrap()
    );
    process::exit(2);
}

fn retrieve_small_options(arg: Chars, options: &mut [bool; NBR_OPTIONS]) {
    for option in arg.skip(1) {
        match option {
            'a' => options[ALL] = true,
            'l' => options[LISTING] = true,
            'R' => options[RECURSIVE] = true,
            'd' => options[DIRECTORY] = true,
            'r' => options[REVERSE] = true,
            't' => options[TIME] = true,
            _ => exit_bad_small_option(option),
        }
    }
}

pub fn parse() -> [bool; NBR_OPTIONS] {
    let mut options = [false; NBR_OPTIONS];

    for arg in env::args().skip(1) {
        let (Some(first_char), Some(second_char)) =
            (arg.chars().nth(0), arg.chars().nth(1))
        else {
            continue;
        };

        if first_char == '-' && second_char == '-' {
            retrieve_big_option(&arg, &mut options);
        } else if first_char == '-' {
            retrieve_small_options(arg.chars(), &mut options);
        }
    }

    options
}
