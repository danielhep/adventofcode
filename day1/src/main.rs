use std::fs;

const CHARACTER_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
fn main() {
    let contents = fs::read_to_string("day1/src/input.txt").expect("File contents not read");
    let lines = contents.split("\n");
    let mut sum = 0;
    for line in lines.into_iter() {
        let mut digit1: Option<u32> = Option::None;
        let mut digit2: Option<u32> = Option::None;
        for character in line.chars() {
            // Look for a digit
            if CHARACTER_DIGITS.contains(&character) {
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

