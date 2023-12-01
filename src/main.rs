use std::io::{self, BufRead};

fn main() {
    let reader = io::stdin().lock();

    let result = reader.lines().filter_map(|l| {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        l.expect("failed to read line").chars().for_each(|c| {
            if c.is_digit(10) {
                first.get_or_insert(c);
                last = Some(c);

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
