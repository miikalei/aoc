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

    for line in reader.lines() {
        let line_numbers: Vec<_> = line
            .expect("Reading a line failed.")
            .chars()
            .filter_map(|char| char.to_digit(10))
            .collect();

        let first = line_numbers.first().unwrap();
        let last = line_numbers.last().unwrap();

        sum += 10 * first + last;
    }

    println!("Total calibration value: {}", sum);
}
