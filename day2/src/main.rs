use regex::Regex;
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("input.txt");
    let file = match File::open(path) {
        Err(why) => panic!("Couln't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let re = Regex::new(r"Game (\d+): (.+)").expect("Regexp should compile.");

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mat = re.captures(&line).expect("Should have a match").extract();
        let (_full, [_game_id, game_result]) = mat;

        let game_sets = game_result.split(';');
        let bags = game_sets.map(Bag::new);

        let min_bag = bags.reduce(|acc, e| Bag::max(&acc, &e)).unwrap();
        let power = min_bag.power();

        sum += power;
    }

    println!("Sum of the powers of the minimal bags: {}", sum);
}

struct Bag {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl Bag {
    pub fn new(game_result: &str) -> Self {
        let green_re = Regex::new(r"(\d+) green").unwrap();
        let blue_re = Regex::new(r"(\d+) blue").unwrap();
        let red_re = Regex::new(r"(\d+) red").unwrap();

        let green = match_count(game_result, green_re);
        let blue = match_count(game_result, blue_re);
        let red = match_count(game_result, red_re);

        BagResult { green, blue, red }
    }

    pub fn fits(&self, other: &Bag) -> bool {
        self.blue >= other.blue && self.green >= other.green && self.red >= other.red
    }

    pub fn max(bag1: &Bag, bag2: &Bag) -> Self {
        Bag {
            red: max(bag1.red, bag2.red),
            green: max(bag1.green, bag2.green),
            blue: max(bag1.blue, bag2.blue),
        }
    }

    pub fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

fn match_count(game_result: &str, regex: Regex) -> u32 {
    regex
        .captures(game_result)
        .map(|cap| {
            let (_full, [cap]) = cap.extract();
            cap.parse().unwrap()
        })
        .unwrap_or(0)
}

type BagResult = Bag;
