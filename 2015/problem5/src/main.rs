use std::fs::File;
use std::io::Read;
use std::path::Path;

const FILE_PATH: &str = "input.txt";

fn main() {
    let path = Path::new(FILE_PATH);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }
    run(&s);
}

fn run(input: &str) {
    let mut nice_count = 0;
    for line in input.lines() {
        if is_nice(line) {
            nice_count += 1;
        }
    }
    println!("Total number of nice lines: {}", nice_count)
}

fn is_nice(line: &str) -> bool {
    contains_non_overlapping_pair_twice(line) && contains_letter_hamburger(line)
}

fn contains_non_overlapping_pair_twice(line: &str) -> bool {
    let re = fancy_regex::Regex::new(r"([[:alpha:]]{2}).*\1").unwrap();
    re.is_match(line).unwrap()
}

fn contains_letter_hamburger(line: &str) -> bool {
    let re = fancy_regex::Regex::new(r"([[:alpha:]]).\1").unwrap();
    re.is_match(line).unwrap()
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'u', 'o'];
fn is_vowel(char: &char) -> bool {
    VOWELS.contains(char)
}

fn contains_three_vowels(line: &str) -> bool {
    // VOWELS.iter().filter(|vowel| line.contains(**vowel)).count() >= 3
    line.chars().filter(is_vowel).count() >= 3
}

fn contains_double_letter(line: &str) -> bool {
    let re = fancy_regex::Regex::new(r"([[:alpha:]])\1").unwrap();
    let ret = re.is_match(line).unwrap();
    println!("Line {} resulting in {}", line, ret);
    ret
}

const SECRET_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];
fn contains_secret_substring(line: &str) -> bool {
    SECRET_SUBSTRINGS.iter().any(|str| line.contains(str))
}
