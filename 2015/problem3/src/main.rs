use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
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
    let mut logger = CoordinateLog::new();
    logger.log(Coordinate(0, 0));
    let mut santa = Mover {
        location: Coordinate(0, 0),
    };
    let mut robo_santa = Mover {
        location: Coordinate(0, 0),
    };
    for (index, char) in input.chars().enumerate() {
        if index % 2 == 0 {
            process_step(&mut santa, char);
            logger.log(santa.location);
        } else {
            process_step(&mut robo_santa, char);
            logger.log(robo_santa.location);
        }
    }
    println!("Total locations visited: {}", logger.count())
}

fn process_step(mover: &mut Mover, char: char) {
    match char {
        '<' => mover.step(Direction::LEFT),
        '>' => mover.step(Direction::RIGHT),
        '^' => mover.step(Direction::UP),
        'v' => mover.step(Direction::DOWN),
        char => panic!("Unknown char: {}", char),
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
struct Mover {
    location: Coordinate,
}

impl Mover {
    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::UP => self.location.1 += 1,
            Direction::DOWN => self.location.1 -= 1,
            Direction::LEFT => self.location.0 -= 1,
            Direction::RIGHT => self.location.0 += 1,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coordinate(i32, i32);
struct CoordinateLog {
    location_set: HashSet<Coordinate>,
}

impl CoordinateLog {
    pub fn log(&mut self, coordinate: Coordinate) {
        self.location_set.insert(coordinate);
    }

    pub fn new() -> Self {
        CoordinateLog {
            location_set: HashSet::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.location_set.len()
    }
}
