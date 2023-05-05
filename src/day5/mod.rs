use std::{collections::HashMap, u8};

use crate::utils::get_data;

fn check_vowel(c: u8) -> Option<u8> {
    match c as char {
        'a' | 'e' | 'i' | 'o' | 'u' => Some(c),
        _ => None,
    }
}

fn check_double(c1: u8, c2: u8) -> bool {
    c1 == c2
}

fn check_bad_string(c1: u8, c2: u8) -> bool {
    matches!(
        (c1 as char, c2 as char),
        ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
    )
}

fn check_line_is_nice(line: &str) -> bool {
    let mut vowels = Vec::new();

    let line_bytes = line.as_bytes();

    if let Some(c) = check_vowel(line_bytes[0]) {
        vowels.push(c);
    }

    let mut double = false;

    line_bytes
        .iter()
        .collect::<Vec<&u8>>()
        .windows(2)
        .all(|window| {
            if let Some(c) = check_vowel(*window[1]) {
                vowels.push(c);
            }

            if check_double(*window[0], *window[1]) {
                double = true;
            }

            if check_bad_string(*window[0], *window[1]) {
                return false;
            }

            true
        })
        && vowels.len() >= 3
        && double
}

pub fn first_part() {
    let data = get_data("data/day5");

    let lines = data.lines();

    let mut nice = 0;

    for line in lines {
        if check_line_is_nice(line) {
            nice += 1;
        }
    }

    println!("Nice strings: {}", nice);
}

fn check_line_is_nice_2(line: &str) -> bool {
    let mut pairs = HashMap::new();

    let line_bytes = line.as_bytes();

    let collection = line_bytes.iter().collect::<Vec<&u8>>();

    let triplet = collection.windows(3).any(|window| window[0] == window[2]);

    if !triplet {
        return false;
    }

    let double_pair = collection.windows(2).enumerate().any(|(i, window)| {
        if pairs.contains_key(&(*window[0], *window[1])) {
            if i - pairs[&(*window[0], *window[1])] > 1 {
                return true;
            }

            return false;
        }

        pairs.insert((*window[0], *window[1]), i);

        false
    });

    double_pair
}

pub fn second_part() {
    let data = get_data("data/day5");

    let lines = data.lines();

    let mut nice = 0;

    for line in lines {
        if check_line_is_nice_2(line) {
            nice += 1;
        }
    }

    println!("Nice strings: {}", nice);
}
