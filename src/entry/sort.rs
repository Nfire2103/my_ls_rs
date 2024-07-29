use super::Entry;
use std::cmp::Ordering;

fn compare_by_time(entry_1: &impl Entry, entry_2: &impl Entry) -> Ordering {
    entry_2.get_mtime().cmp(&entry_1.get_mtime())
}

fn compare_by_name(entry_1: &impl Entry, entry_2: &impl Entry) -> Ordering {
    let mut str_1 = entry_1.get_name().to_lowercase();
    let mut str_2 = entry_2.get_name().to_lowercase();

    str_1 = str_1.chars().skip_while(|&c| c == '.').collect();
    str_2 = str_2.chars().skip_while(|&c| c == '.').collect();

    str_1.cmp(&str_2)
}

pub fn sort_entries(entries: &mut Vec<impl Entry>, sort_by_time: bool) {
    if sort_by_time {
        entries.sort_by(compare_by_time);
    } else {
        entries.sort_by(compare_by_name);
    }
}
