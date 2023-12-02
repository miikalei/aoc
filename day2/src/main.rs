use regex::Regex;
use std::{
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

    const BAG: Bag = Bag {
        red: 12,
        blue: 14,
        green: 13,
    };

    let reader = BufReader::new(file);

    let re = Regex::new(r"Game (\d+): (.+)").expect("Regexp should compile.");

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mat = re.captures(&line).expect("Should have a match").extract();
        let (_full, [game_id, game_result]) = mat;

        let game_id: u32 = game_id.parse().expect("game_id should be an integer.");
        let mut game_sets = game_result.split(';');

        let game_failed = game_sets.any(|set| {
            let bag_result = BagResult::new(set);
            !BAG.fits(&bag_result)
        });
        if !game_failed {
            sum += game_id;
        }
    }

    println!("Sum of successful game ids: {}", sum);
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
