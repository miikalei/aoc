use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }

    run(&s);
}

fn run(input: &str) {
    let mut distance_dict = DistanceDict::new();
    let mut city_set = HashSet::new();
    for line in input.lines() {
        let datum = DistanceDatum::from(line);
        city_set.insert(datum.from.clone());
        city_set.insert(datum.to.clone());
        distance_dict.insert(&datum);
    }
    // Solve the problem using distanceDict
    let best_perm = city_set
        .iter()
        .permutations(city_set.len())
        .max_by_key(|city_list| score_list(city_list, &distance_dict))
        .unwrap();
    println!(
        "Shortest distance: {}",
        score_list(&best_perm, &distance_dict)
    )
}

fn score_list(city_list: &[&String], distance_dict: &DistanceDict) -> i32 {
    let iter1 = city_list.iter();
    let mut iter2 = iter1.clone();
    iter2.next();
    let iter3 = iter1.zip(iter2);
    let edges: Vec<_> = iter3.clone().collect();
    let distances = iter3.map(|edge| distance_dict.get_distance(edge.0, edge.1));
    let distance: i32 = distances.clone().sum();
    println!("Calculating distance of {:?}", city_list);
    println!("Edges are {:?}", edges);
    println!("Distances are {:?}", distances.collect_vec());
    println!("Got distance {}", distance);
    distance
}

#[derive(Debug)]
struct DistanceDatum {
    from: String,
    to: String,
    distance: i32,
}

impl From<&str> for DistanceDatum {
    fn from(value: &str) -> Self {
        let (left_side, right_side) = value.split_once(" = ").unwrap();
        let distance: i32 = right_side.parse().unwrap();
        let (from, to) = left_side.split_once(" to ").unwrap();

        DistanceDatum {
            from: from.to_string(),
            to: to.to_string(),
            distance,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Edge(String, String);

struct DistanceDict {
    dict: HashMap<Edge, i32>,
}

impl DistanceDict {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }

    pub fn insert(&mut self, datum: &DistanceDatum) {
        self.dict.insert(
            Edge(datum.from.to_string(), datum.to.to_string()),
            datum.distance,
        );
        self.dict.insert(
            Edge(datum.to.to_string(), datum.from.to_string()),
            datum.distance,
        );
    }

    pub fn get_distance(&self, from: &String, to: &String) -> i32 {
        let temp = Edge(from.to_string(), to.to_string());
        self.dict[&temp]
    }
}
