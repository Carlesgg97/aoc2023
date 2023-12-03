use std::io::{self, BufRead as _};

pub struct LineNumber {
    pub number: u32,
    pub start_index: usize,
    pub end_index: usize,
    pub valid: bool
}

const EMPTY: char = '.';

fn main() {
    let reader = io::stdin().lock();

    let mut prev_line_symbols: Vec<usize> = vec![];
    let mut prev_line_numbers: Vec<LineNumber> = vec![];

    let mut valid_numbers: Vec<u32> = vec![];

    reader.lines().for_each(|l| {
        let binding = l.expect("failed to read line");
        let l = binding.as_str();

        let mut line_symbols: Vec<usize> = vec![];
        let mut line_numbers: Vec<LineNumber> = vec![];

        let mut curr_number = String::new();

        l.char_indices().for_each(|(idx, c)| {
            if c.is_digit(10) {
                curr_number.push(c);
                return;
            }

            if !curr_number.is_empty() {
                let number = curr_number.parse::<u32>().expect("failed to parse number");
                line_numbers.push(LineNumber {
                    number,
                    start_index: idx - curr_number.len(),
                    end_index: idx - 1,
                    valid: false
                });
                curr_number.clear();

                let ln: &mut LineNumber = line_numbers.last_mut().unwrap();

                // Check for prev line symbols
                let prev_line_found = prev_line_symbols.iter().any(|i| {
                    let prev = if ln.start_index > 0 {
                        (ln.start_index - 1) <= *i
                    } else {
                        ln.start_index <= *i
                    };
                    prev && *i <= (ln.end_index + 1)
                });
                if prev_line_found { ln.valid = true; }

                // Check for before symbol
                if line_symbols.last().is_some() {
                    let last_idx = line_symbols.last().unwrap();
                    if (last_idx + 1) == ln.start_index {
                        ln.valid = true;
                    }
                }
            }

            // Ocurrence of a symbol
            if c != EMPTY {
                line_symbols.push(idx);

                // Check after symbol
                if line_numbers.last().is_some() {
                    let ln: &mut LineNumber = line_numbers.last_mut().unwrap();
                    if (ln.end_index + 1) == idx {
                        ln.valid = true;
                    }
                }

                // Check for prev line numbers
                prev_line_numbers.iter_mut().for_each(|ln| {
                    let found = if ln.start_index > 0 {
                        (ln.start_index - 1) <= idx
                    } else {
                        ln.start_index <= idx
                    } && idx <= (ln.end_index + 1);
                    if found { ln.valid = true; }
                });
                return;
            }
        });

        // check curr number
        if !curr_number.is_empty() {
            let number = curr_number.parse::<u32>().expect("failed to parse number");
            line_numbers.push(LineNumber {
                number,
                start_index: l.len() - curr_number.len(),
                end_index: l.len() - 1,
                valid: false
            });
            curr_number.clear();

            let ln: &mut LineNumber = line_numbers.last_mut().unwrap();

            // Check for prev line symbols
            let prev_line_found = prev_line_symbols.iter().any(|i| {
                let prev = if ln.start_index > 0 {
                    (ln.start_index - 1) <= *i
                } else {
                    ln.start_index <= *i
                };
                prev && *i <= (ln.end_index + 1)
            });
            if prev_line_found { ln.valid = true; }

            // Check for before symbol
            if line_symbols.last().is_some() {
                let last_idx = line_symbols.last().unwrap();
                if (last_idx + 1) == ln.start_index {
                    ln.valid = true;
                }
            }
        }

        prev_line_numbers.iter().for_each(|ln| {
            if ln.valid {
                valid_numbers.push(ln.number);
            }
        });
       
        prev_line_symbols = line_symbols;
        prev_line_numbers = line_numbers;
    });

    prev_line_numbers.iter().for_each(|ln| {
        if ln.valid {
            valid_numbers.push(ln.number);
        }
    });

    println!("{}", valid_numbers.iter().sum::<u32>());
}