#[derive(Debug,Clone)]
pub enum DigitString {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl DigitString {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DigitString::One => "one",
            DigitString::Two => "two",
            DigitString::Three => "three",
            DigitString::Four => "four",
            DigitString::Five => "five",
            DigitString::Six => "six",
            DigitString::Seven => "seven",
            DigitString::Eight => "eight",
            DigitString::Nine => "nine",
        }
    }

    pub fn as_char(&self) -> char {
        match *self {
            DigitString::One => '1',
            DigitString::Two => '2',
            DigitString::Three => '3',
            DigitString::Four => '4',
            DigitString::Five => '5',
            DigitString::Six => '6',
            DigitString::Seven => '7',
            DigitString::Eight => '8',
            DigitString::Nine => '9',
        }
    }

    pub fn iterator() -> DigitStringIterator {
        DigitStringIterator { current_variant: None }
    }
}

// Implement an iterator over DigitString
pub struct DigitStringIterator {
    current_variant: Option<DigitString>,
}

impl Iterator for DigitStringIterator {
    type Item = DigitString;

    fn next(&mut self) -> Option<Self::Item> {
        let next_variant = match self.current_variant {
            Some(ref current) => match *current {
                DigitString::One => Some(DigitString::Two),
                DigitString::Two => Some(DigitString::Three),
                DigitString::Three => Some(DigitString::Four),
                DigitString::Four => Some(DigitString::Five),
                DigitString::Five => Some(DigitString::Six),
                DigitString::Six => Some(DigitString::Seven),
                DigitString::Seven => Some(DigitString::Eight),
                DigitString::Eight => Some(DigitString::Nine),
                DigitString::Nine => None,
            },
            None => Some(DigitString::One),
        };

        self.current_variant = next_variant.clone();
        next_variant
    }
}
