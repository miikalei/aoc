use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    path::Path,
};

use iter_tools::Itertools;

const FILE_PATH: &str = "input.txt";

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
    let mut dist_dict = DistanceDict::new();
    for line in s.lines() {
        dist_dict.insert(line);
    }
    let mut person_set: HashSet<String> = HashSet::new();
    for Pair(target, _) in dist_dict.dist_dict.keys() {
        person_set.insert(target.to_string());
    }
    println!("The collected data: {:?}", &dist_dict);
    println!("The person set: {:?}", person_set);
    let min_score = person_set
        .iter()
        .permutations(person_set.len())
        .map(|comb| score_comb(&comb, &dist_dict))
        .max()
        .unwrap();
    println!("Minimum score: {}", min_score)
}

fn score_comb(comb: &[&String], dist_dict: &DistanceDict) -> i32 {
    let iter1 = comb.iter();
    let mut iter2 = comb.iter().clone();
    iter2.next();
    let iter3 = iter1.zip(iter2);
    let scores =
        iter3.map(|edge| score_edge(&Pair(edge.0.to_string(), edge.1.to_string()), dist_dict));
    let first = comb.first().unwrap();
    let last = comb.last().unwrap();
    let a: i32 = scores.sum();
    let b = score_edge(&Pair(first.to_string(), last.to_string()), dist_dict);
    a + b
}

fn score_edge(edge: &Pair, dist_dict: &DistanceDict) -> i32 {
    let a = dist_dict.dist_dict[edge];
    let b = dist_dict.dist_dict[&Pair(edge.1.to_string(), edge.0.to_string())];
    a + b
}

#[derive(Debug)]
struct DistanceDict {
    pub dist_dict: HashMap<Pair, i32>,
}

impl DistanceDict {
    fn new() -> Self {
        DistanceDict {
            dist_dict: HashMap::new(),
        }
    }

    fn insert(&mut self, line: &str) {
        let (target, rest) = line.split_once(" would ").unwrap();
        let (valence, rest) = rest.split_once(' ').unwrap();
        let (amount, rest) = rest
            .split_once(" happiness units by sitting next to ")
            .unwrap();
        let amount: i32 = amount.parse().unwrap();
        let amount = match valence {
            "gain" => amount,
            "lose" => -amount,
            _ => panic!("Unknown valence."),
        };
        let source = rest.strip_suffix('.').unwrap();

        self.dist_dict
            .insert(Pair(target.to_string(), source.to_string()), amount);
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Pair(String, String);
