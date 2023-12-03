use std::{io::{self, BufRead as _}, collections::HashSet};

#[derive(PartialEq, Eq, Hash)]
pub struct LineNumber {
    pub number: u32,
    pub start_index: usize,
    pub end_index: usize,
}

const GEAR: char = '*';
const GEAR_ARMS: usize = 2;

fn main() {
    let reader = io::stdin().lock();

    let mut muls = vec![];
    let mut line_size = 0;

    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    lines.iter().enumerate().for_each(|(line_num, line)| {
        if line_size == 0 { line_size = line.len(); }

        line.char_indices().for_each(|(char_num, c)| {
            if c == GEAR {
                // Search and parse all adjacent numbers
                let mut nums = vec![];
                for i in -1..=1 {
                    if line_num == 0 && i == -1 || add(line_num, i) >= lines.len() { continue; }
                    for j in -1..=1 {
                        if char_num == 0 && j == -1 || add(char_num, j) >= line_size { continue; }
                        let num = parse_number(&lines[add(line_num, i)], add(char_num, j));
                        if num.is_some() {
                            nums.push(num.unwrap());
                        }
                    }
                }

                let unique_nums: HashSet<_> = nums.into_iter().collect();

                if unique_nums.len() == GEAR_ARMS {
                    let mul = unique_nums.iter().map(|n| n.number).fold(1, |acc, x| {
                        acc * x
                    });
                    muls.push(mul);
                }
            }
        });
    });
    
    println!("{}", muls.iter().sum::<u32>());
}

fn parse_number(s: &String, i: usize) -> Option<LineNumber> {
    if !s.chars().nth(i).unwrap().is_digit(10) { return None; }

    let mut str_start: usize = i;
    let mut str_end: usize = i;

    let mut string_back = String::new();
    let mut string_front = String::new();

    for k in (0..=i-1).rev() {
        let c = s.chars().nth(k).unwrap();
        if c.is_digit(10) {
            string_back.push(c);
            str_start = k;
        } else {
            break;
        }
    }   
    
    for k in i+1..s.len() {
        let c = s.chars().nth(k).unwrap();
        if c.is_digit(10) {
            string_front.push(c);
            str_end = k;
        } else {
            break;
        }
    }
    
    let back_str: String = string_back.chars().rev().collect();

    let num_str = back_str + &s.chars().nth(i).unwrap().to_string() + &string_front;

    return Some(LineNumber {
        number: num_str.parse().unwrap(),
        start_index: str_start,
        end_index: str_end,
    });
}

fn add(a: usize, b: i32) -> usize {
    if b < 0 {
        a - b.abs() as usize
    } else {
        a + b as usize
    }
}