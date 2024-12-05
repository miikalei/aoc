use regex::Regex;
use std::io::Read;
use std::{fs::File, path::Path};

fn main() {
    let input = read_file_to_string("input.txt");

    let row_length = input.lines().next().unwrap().len() as i32 + 1; // Add for newline char

    let symbols = find_symbols_from_string(&input);
    let numbers = find_numbers_from_string(&input);

    println!("Row length: {}", row_length);

    let mut sum = 0;

    for symbol in symbols.iter().filter(|symbol| symbol.symbol == '*') {
        let neighbor_numbers: Vec<_> = numbers
            .iter()
            .filter(|number| number.is_next_to(symbol.index, row_length))
            .collect();
        if neighbor_numbers.len() == 2 {
            println!(
                "Found neighbor numbers: {}, {}",
                neighbor_numbers[0].number, neighbor_numbers[1].number
            );
            sum += neighbor_numbers[0].number * neighbor_numbers[1].number
        }
    }

    println!("The sum of gear powers: {}", sum);
}

#[derive(Debug)]
struct Symbol {
    pub symbol: char,
    pub index: u32,
}

#[derive(Debug)]
struct Number {
    pub number: u32,
    pub start_index: u32,
    pub end_index: u32,
}

impl Number {
    fn overlaps(&self, index: i32) -> bool {
        if index < 0 {
            return false;
        }
        let index = index as u32;
        self.start_index <= index && index < self.end_index
    }

    fn is_next_to(&self, index: u32, row_length: i32) -> bool {
        let index = index as i32;
        self.overlaps(index - row_length - 1)
            || self.overlaps(index - row_length)
            || self.overlaps(index - row_length + 1)
            || self.overlaps(index - 1)
            || self.overlaps(index + 1)
            || self.overlaps(index + row_length - 1)
            || self.overlaps(index + row_length)
            || self.overlaps(index + row_length + 1)
    }
}

fn find_symbols_from_string(input: &str) -> Vec<Symbol> {
    let re = Regex::new(r"[^\n.[0-9]]").expect("Regexp should have compiled.");

    re.find_iter(input)
        .map(|mat| Symbol {
            symbol: mat.as_str().chars().next().unwrap(),
            index: mat.start() as u32,
        })
        .collect()
}

fn find_numbers_from_string(input: &str) -> Vec<Number> {
    let re = Regex::new(r"[0-9]+").expect("Regexp should have compiled.");

    re.find_iter(input)
        .map(|mat| Number {
            number: mat
                .as_str()
                .parse()
                .expect("Did not manage to parse number."),
            start_index: mat.start() as u32,
            end_index: mat.end() as u32,
        })
        .collect()
}

fn read_file_to_string(filename: &str) -> String {
    let path = Path::new(filename);
    let mut file = match File::open(path) {
        Err(why) => panic!("Couln't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}
