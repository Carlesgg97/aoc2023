use std::{io::{self, BufRead as _}, collections::HashSet};

fn main() {
    let reader = io::stdin().lock();

    let result = reader.lines().map(|l| {
        let binding = l.expect("failed to read line");
        let l = binding.as_str();

        let (_, game_str) = l.split_once(": ").expect("failed to split game id separator");
        let (wins_str, plays_str) = game_str.split_once(" | ").expect("failed to split game separator");

        let winning_numbers = wins_str.split(' ').filter(|s| !s.is_empty()).map(|n| n.parse::<u32>().expect("failed to parse winning number")).collect::<HashSet<u32>>();

        plays_str.split(' ').filter(|s| !s.is_empty())
            .map(|n| n.parse::<u32>().expect("failed to parse play number"))
            .filter(|n| winning_numbers.contains(&n)).count()
    })
    .filter(|c| *c > 0)
    .map(|c| 2_usize.pow(c as u32 - 1))
    .sum::<usize>();

    println!("{}", result);
}