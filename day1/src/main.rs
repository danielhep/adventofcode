use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("day1/src/input.txt").expect("File contents not read");
    let lines = contents.split("\n");
    let mut sum = 0;

    let first_digit_re = Regex::new(r"^.*?(\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))").unwrap();
    let last_digit_re = Regex::new(r".*(\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)).*$").unwrap();
    for line in lines.into_iter() {
        let mut digit1 = first_digit_re.captures(line).unwrap().get(1).unwrap().as_str();
        let mut digit2 = last_digit_re.captures(line).unwrap().get(1).unwrap().as_str();
        for character in line.chars() {
            // Look for a digit
            if character.is_digit(10) {
                if digit1.is_none() {
                    digit1 = Option::Some(character.to_digit(10).unwrap())
                } else {
                    digit2 = Option::Some(character.to_digit(10).unwrap())
                }
            }
        }
        sum += digit1.unwrap() * 10 + digit2.unwrap_or(digit1.unwrap());
    }

    println!("Total value: {}", sum);
}

