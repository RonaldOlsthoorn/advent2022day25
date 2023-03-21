
use std::{io::{BufReader, BufRead}, fs::File, vec, num, fmt};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SNAFUDigit {
    Zero,
    One,
    Two,
    MinusOne,
    MinusTwo
}

impl TryFrom<& '_ char> for SNAFUDigit {

    type Error = &'static str;

    fn try_from(value: & '_ char) -> Result<Self, Self::Error> {
        match value {
            '1' => {Ok(SNAFUDigit::One)},
            '2' => {Ok(SNAFUDigit::Two)},
            '0' => {Ok(SNAFUDigit::Zero)},
            '-' => {Ok(SNAFUDigit::MinusOne)},
            '=' => {Ok(SNAFUDigit::MinusTwo)},
            _ => {Err("Can only convert for the symbols 0, 1, 2, - and =")},
        }
    }
}

impl TryFrom<& '_ i64> for SNAFUDigit {

    type Error = &'static str;

    fn try_from(value: & '_ i64) -> Result<Self, Self::Error> {

        match value {
            1 => {Ok(SNAFUDigit::One)},
            2 => {Ok(SNAFUDigit::Two)},
            0 => {Ok(SNAFUDigit::Zero)},
            -1 => {Ok(SNAFUDigit::MinusOne)},
            -2 => {Ok(SNAFUDigit::MinusTwo)},
            _ => {Err("Can only convert from values -2, -1, 0, 1 or 2")},
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct SNAFUNumber {
    digits: Vec<SNAFUDigit>
}

impl fmt::Debug for SNAFUNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut s_rep = String::new();

        for snafu_digit in self.digits.iter() {
            match snafu_digit {
                SNAFUDigit::Zero => s_rep.push('0'),
                SNAFUDigit::One => s_rep.push('1'),
                SNAFUDigit::Two => s_rep.push('2'),
                SNAFUDigit::MinusOne => s_rep.push('-'),
                SNAFUDigit::MinusTwo => s_rep.push('=')
            }
        }

        write!(f, "SNAFU Number {}", s_rep)
    }
}

impl Into<usize> for SNAFUNumber {

    fn into(self) -> usize {
        let mut res: i64 = 0;

        for (place, digit) in self.digits.iter().rev().enumerate() {
            match digit {
                SNAFUDigit::Zero => {},
                SNAFUDigit::One => res += 5_i64.pow(place as u32),
                SNAFUDigit::Two => res += 2 * 5_i64.pow(place as u32),
                SNAFUDigit::MinusOne => res -= 5_i64.pow(place as u32),
                SNAFUDigit::MinusTwo => res -= 2 * 5_i64.pow(place as u32)
            }
        }

        res as usize
    }
}

impl From<& '_ usize> for SNAFUNumber {

    fn from(binary_value: & '_ usize) -> Self {
        let mut digits = vec![];

        let mut number_of_digits = 1;
        
        while 5usize.pow(number_of_digits) / 2 < *binary_value {
            number_of_digits += 1;
        }

        Self::add_next_digit(&mut digits, *binary_value as i64, number_of_digits);

        return SNAFUNumber { digits };
    }
}

impl TryFrom<& '_ String> for SNAFUNumber {

    type Error = &'static str;

    fn try_from(symbols: & '_ String) -> Result<Self, Self::Error> {

        let mut digits = vec![];

        for c in symbols.chars() {
            if let Ok(digit) = SNAFUDigit::try_from(&c) {
                digits.push(digit);
            } else {
                return Err("Encountered illegal symbol. Legal symbols =, -, 0, 1, 2");
            }
        }

        Ok(SNAFUNumber{digits})
    }
}

impl SNAFUNumber {

    fn add_next_digit(digits: &mut Vec<SNAFUDigit>, residue: i64, number_of_digits: u32) {

        if number_of_digits == 1 {
            digits.push(SNAFUDigit::try_from(&residue).unwrap());
            return;
        }

        if residue < (-5_i64.pow(number_of_digits) / 2) + 5_i64.pow(number_of_digits - 1) {
            digits.push(SNAFUDigit::MinusTwo);
            Self::add_next_digit(digits, residue + 2 * 5_i64.pow(number_of_digits - 1), number_of_digits - 1);
        } else if residue < (-5_i64.pow(number_of_digits) / 2) + 2 * 5_i64.pow(number_of_digits - 1){
            digits.push(SNAFUDigit::MinusOne);
            Self::add_next_digit(digits, residue + 5_i64.pow(number_of_digits - 1), number_of_digits - 1);
        } else if residue <= (5_i64.pow(number_of_digits) / 2) - 2 * 5_i64.pow(number_of_digits - 1) {
            digits.push(SNAFUDigit::Zero);
            Self::add_next_digit(digits, residue, number_of_digits - 1);
        } else if residue <= (5_i64.pow(number_of_digits) / 2) - 5_i64.pow(number_of_digits - 1) {
            digits.push(SNAFUDigit::One);
            Self::add_next_digit(digits, residue - 5_i64.pow(number_of_digits - 1), number_of_digits - 1);
        } else {
            digits.push(SNAFUDigit::Two);
            Self::add_next_digit(digits, residue - 2 * 5_i64.pow(number_of_digits - 1), number_of_digits - 1);
        }
    }
}



fn main() {
   
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();

    let sum = lines.iter().map(|line| {let d: usize = SNAFUNumber::try_from(line).unwrap().into(); d}).fold(0, |acc, x| acc + x);

    println!("Sum in normal number {}", sum);
    println!("Sum in SNAFU {:?}", SNAFUNumber::from(&sum));
}