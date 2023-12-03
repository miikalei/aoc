use regex::Regex;
use std::io::Read;
use std::{fs::File, path::Path};

fn main() {
    let input = read_file_to_string("input.txt");

    let mut sum = 0;
    let number_re = Regex::new(r"\d+").unwrap();
    let row_length = input.lines().next().unwrap().len() as i32 + 1; // Add for newline char

    println!("Row length: {}", row_length);

    for mat in number_re.find_iter(&input) {
        let number = mat.as_str().parse::<u32>().unwrap();
        let start = mat.start() as i32;
        let end = mat.end() as i32;

        let has_symbol_near = has_symbol_near(&input, start, end, row_length);

        if has_symbol_near {
            println!("YES: {}; {}; {};", number, start, end);
            sum += number;
        } else {
            println!("NO :{}; {}; {};", number, start, end);
        }
    }

    println!("The sum of numbers with a symbol near them: {}", sum);
}

fn has_symbol_near(input: &str, start: i32, end: i32, row_length: i32) -> bool {
    // Line above

    for i in start - 1..end + 1 {
        if has_symbol_at(input, i - row_length) {
            return true;
        }
    }

    // End points
    if has_symbol_at(input, start - 1) {
        return true;
    }
    if has_symbol_at(input, end) {
        return true;
    }

    // Line below
    for i in start - 1..end + 1 {
        if has_symbol_at(input, i + row_length) {
            return true;
        }
    }
    false
}

fn has_symbol_at(input: &str, index: i32) -> bool {
    if index < 0 {
        return false;
    }
    let char = input.chars().nth(index.try_into().unwrap());
    !matches!(
        char,
        None | Some('.')
            | Some('1')
            | Some('2')
            | Some('3')
            | Some('4')
            | Some('5')
            | Some('6')
            | Some('7')
            | Some('8')
            | Some('9')
            | Some('0')
            | Some('\n')
    )
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
