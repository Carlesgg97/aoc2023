use std::io::{self, BufRead as _};

fn main() {
    let reader = io::stdin().lock();

    let result = reader.lines().map(|l| {
        let binding = l.expect("failed to read line");
        let l = binding.as_str();

        let (_, plays_str) = l.get(5..).expect("failed to read game id")
                                    .split_once(": ").expect("failed to split game separator");

        let game = plays_str.split("; ").map(|p| { // each "play" in a game
            let mut red_max: u32 = 0;
            let mut green_max: u32 = 0;
            let mut blue_max: u32 = 0;

            p.split(", ").for_each(|cr| { // each colour result
                let (n_str, colour_str) = cr.split_once(' ').expect("failed to parse colour result");
                let n = n_str.parse::<u32>().expect("failed to parse colour #");

                match colour_str {
                    "red" => {
                        red_max = red_max.max(n)
                    },
                    "green" => {
                        green_max = green_max.max(n)
                    },
                    "blue" => {
                        blue_max = blue_max.max(n)
                    },
                    _ => ()
                };
            });

            (red_max, green_max, blue_max)
        }).fold((0,0,0), |acc, x| {
            (acc.0.max(x.0), acc.1.max(x.1), acc.2.max(x.2))
        });

        game.0 * game.1 * game.2
    }).sum::<u32>();

    println!("{}", result);
}