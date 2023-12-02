use std::io::{self, BufRead as _};

const BAG_RED_CUBES: u32 = 12;
const BAG_GREEN_CUBES: u32 = 13;
const BAG_BLUE_CUBES: u32 = 14;

fn main() {
    let reader = io::stdin().lock();

    let result = reader.lines().filter_map(|l| {
        let binding = l.expect("failed to read line");
        let l = binding.as_str();

        let (game_id_str, plays_str) = l.get(5..)?.split_once(": ").expect("failed to split game separator");

        let game_id = game_id_str.parse::<u32>().expect("failed to parse game id");

        let valid = plays_str.split("; ").all(|p| { // all "plays" in a game satisfy
            p.split(", ").all(|cr| { // all colour results satisfy
                let (n_str, colour_str) = cr.split_once(' ').expect("failed to parse colour result");
                let n = n_str.parse::<u32>().expect("failed to parse colour #");

                return match colour_str {
                    "red" => n <= BAG_RED_CUBES,
                    "green" => n <= BAG_GREEN_CUBES,
                    "blue" => n <= BAG_BLUE_CUBES,
                    _ => false
                };
            })
        });

        if valid {Some(game_id)} else {None}
    }).sum::<u32>();

    println!("{}", result);
}