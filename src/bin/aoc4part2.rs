use std::{io::{self, BufRead as _}, collections::{HashSet, HashMap}};

fn main() {
    let reader = io::stdin().lock();

    let mut copies = HashMap::<usize, usize>::new();

    let result = reader.lines().enumerate().map(|(idx, l)| {
        let binding = l.expect("failed to read line");
        let l = binding.as_str();

        let (_, game_str) = l.split_once(": ").expect("failed to split game id separator");
        let (wins_str, plays_str) = game_str.split_once(" | ").expect("failed to split game separator");

        let winning_numbers = wins_str.split(' ').filter(|s| !s.is_empty()).map(|n| n.parse::<u32>().expect("failed to parse winning number")).collect::<HashSet<u32>>();

        let current_copies = copies.get(&idx).cloned().unwrap_or(0);

        let wins = plays_str.split(' ').filter(|s| !s.is_empty())
            .map(|n| n.parse::<u32>().expect("failed to parse play number"))
            .filter(|n| winning_numbers.contains(&n)).count();

        for i in 0..wins {
            *copies.entry(idx + i + 1).or_default() += 1 + current_copies;
        }

        copies.remove(&idx);
        current_copies + 1
    })
    .sum::<usize>();

    println!("{}", result);
}