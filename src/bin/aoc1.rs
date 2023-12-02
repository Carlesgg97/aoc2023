use std::io::{self, BufRead};

use aoc2023::digit_string::DigitString;

fn main() {
    let reader = io::stdin().lock();

    let result = reader.lines().filter_map(|l| {
        let binding = l.expect("failed to read line");
        let l = binding.as_str();

        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        l.char_indices().for_each(|(offset, c)| {
            if c.is_digit(10) {
                first.get_or_insert(c);
                last = Some(c);
                return;
            }

            let num_match = DigitString::iterator().find_map(|digit_string| {
                if l.get(offset..)?.starts_with(digit_string.as_str()) {
                    return Some(digit_string.as_char())
                }
                None
            });

            if let Some(d) = num_match {
                first.get_or_insert(d);
                last = Some(d);
            }
        });

        if first.is_none() {
            return None;
        }

        let first_char = first.unwrap().to_string();
        let last_char = last.or(first).unwrap().to_string();

        Some((first_char + &last_char).parse::<u64>())
    }).flatten().sum::<u64>();

    println!("{}", result);
}
