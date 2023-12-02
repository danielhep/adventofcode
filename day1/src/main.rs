use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    // Make the look up table
    let string_int_vals: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let contents = fs::read_to_string("day1/src/input.txt").expect("File contents not read");
    let lines = contents.split("\n");
    let mut sum = 0;

    let first_digit_re =
        Regex::new(r"^.*?(\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))")
            .unwrap();
    let last_digit_re =
        Regex::new(r".*(\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)).*$")
            .unwrap();
    let regexes = [first_digit_re, last_digit_re];
    for line in lines.into_iter() {
        let captures = regexes.clone().map(|regex| {
            regex
                .captures(line)
                .unwrap()
                .get(1)
                .expect("no digit found!")
                .as_str()
        });

        let digits = captures.map(|c| {
            if c.chars().nth(0).unwrap().is_digit(10) {
                c.chars().nth(0).unwrap().to_digit(10).unwrap()
            } else {
                *string_int_vals.get(c).unwrap()
            }
        });

        sum += digits[0] * 10 + digits[1];
    }

    println!("Total value: {}", sum);
}
