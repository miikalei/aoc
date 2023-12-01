use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("input.txt");

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut sum = 0;

    let first_re =
        Regex::new(r"([[:digit:]]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let last_re =
        Regex::new(r"^.*([[:digit:]]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let first_match = &first_re.captures(&line).unwrap()[1];
        let last_match = &last_re.captures(&line).unwrap()[1];

        let value = 10 * string_to_value(first_match) + string_to_value(last_match);
        println!("{}", value);
        sum += value;
    }

    println!("Total calibration value: {}", sum);
}

fn string_to_value(str: &str) -> u32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        digit => digit.chars().next().unwrap().to_digit(10).unwrap(),
    }
}
