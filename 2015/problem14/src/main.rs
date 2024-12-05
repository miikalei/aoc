use std::{cmp, collections::HashMap, fs::File, hash::Hash, io::Read, path::Path};

const FILE_PATH: &str = "input.txt";
const COMPETITION_LENGTH: i32 = 2503;

fn main() {
    let path = Path::new(FILE_PATH);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }

    run(&s);
}

fn run(s: &str) {
    let mut dist_dict = ReindeerStatsDict::new();
    let mut score_dict: HashMap<String, i32> = HashMap::new();
    for line in s.lines() {
        dist_dict.insert(line);
    }
    println!("Dict: {:?}", &dist_dict.dict);
    for t in 1..=COMPETITION_LENGTH {
        let best_dist = dist_dict
            .dict
            .values()
            .map(|reindeer| score_reindeer(reindeer, t))
            .max()
            .unwrap();
        for (reindeer_name, reindeer_stats) in &dist_dict.dict {
            if score_reindeer(reindeer_stats, t) == best_dist {
                // increase score
                score_dict
                    .entry(reindeer_name.to_string())
                    .and_modify(|e| {
                        *e += 1;
                    })
                    .or_insert(1);
            }
        }
    }
    let best_score = score_dict.values().max().unwrap();
    println!("Best score by any reindeer: {}", best_score);
}

fn score_reindeer(reindeer: &ReindeerStats, t: i32) -> i32 {
    let cycle_length = reindeer.span + reindeer.rest;
    let partial_cycle_length = t % cycle_length;
    let full_cycles = (t - partial_cycle_length) / cycle_length;
    full_cycles * reindeer.span * reindeer.speed
        + cmp::min(partial_cycle_length, reindeer.span) * reindeer.speed
}

#[derive(Debug)]
struct ReindeerStatsDict {
    pub dict: HashMap<String, ReindeerStats>,
}

impl ReindeerStatsDict {
    fn new() -> Self {
        ReindeerStatsDict {
            dict: HashMap::new(),
        }
    }

    pub fn insert(&mut self, line: &str) {
        let (name, rest) = line.split_once(" can fly ").unwrap();
        let (speed, rest) = rest.split_once(" km/s for ").unwrap();
        let (span, rest) = rest
            .split_once(" seconds, but then must rest for ")
            .unwrap();
        let rest = rest.strip_suffix(" seconds.").unwrap();

        self.dict.insert(
            name.to_string(),
            ReindeerStats {
                speed: speed.parse().unwrap(),
                span: span.parse().unwrap(),
                rest: rest.parse().unwrap(),
            },
        );
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct ReindeerStats {
    speed: i32,
    span: i32,
    rest: i32,
}
