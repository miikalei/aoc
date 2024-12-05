use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

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
    let mut light_field = LightField::new();
    for line in input.lines() {
        let command: Command = line.parse().unwrap();
        println!("Command: {:?}", command);
        light_field.accept(&command)
    }
    println!("Total number of lights on: {}", light_field.count())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(i32, i32);

#[derive(Clone, Copy)]
struct Range(Coord, Coord);

#[derive(Debug)]
enum CommandType {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError {}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command_type, s) = match s {
            s if s.starts_with("turn on ") => {
                (CommandType::TurnOn, s.strip_prefix("turn on").unwrap())
            }
            s if s.starts_with("turn off ") => {
                (CommandType::TurnOff, s.strip_prefix("turn off").unwrap())
            }
            s if s.starts_with("toggle ") => {
                (CommandType::Toggle, s.strip_prefix("toggle").unwrap())
            }
            _ => panic!("Unknown command."),
        };

        let (from_raw, to_raw) = s.split_once("through").unwrap();
        let from: Coord = from_raw.parse().unwrap();
        let to: Coord = to_raw.parse().unwrap();

        Ok(Command {
            command_type,
            from,
            to,
        })
    }
}

impl FromStr for Coord {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').ok_or(Self::Err {})?;
        let x_fromstr = x.parse::<i32>().map_err(|_| Self::Err {})?;
        let y_fromstr = y.parse::<i32>().map_err(|_| Self::Err {})?;
        Ok(Coord(x_fromstr, y_fromstr))
    }
}

#[derive(Debug)]
struct Command {
    pub command_type: CommandType,
    pub from: Coord,
    pub to: Coord,
}

struct LightField {
    light_map: HashMap<Coord, i32>,
}

impl LightField {
    fn new() -> Self {
        let mut light_map = HashMap::new();
        for x in 0..1000 {
            for y in 0..1000 {
                light_map.insert(Coord(x, y), 0);
            }
        }
        LightField { light_map }
    }

    pub fn accept(&mut self, command: &Command) {
        let range = Range(command.from, command.to);
        match command.command_type {
            CommandType::Toggle => self.toggle_range(&range),
            CommandType::TurnOn => self.turn_on_range(&range),
            CommandType::TurnOff => self.turn_off_range(&range),
        }
    }

    fn turn_on_range(&mut self, range: &Range) {
        for x in (range.0 .0)..=(range.1 .0) {
            for y in range.0 .1..=range.1 .1 {
                self.turn_on(&Coord(x, y))
            }
        }
    }
    fn turn_off_range(&mut self, range: &Range) {
        for x in range.0 .0..=range.1 .0 {
            for y in range.0 .1..=range.1 .1 {
                self.turn_off(&Coord(x, y))
            }
        }
    }
    fn toggle_range(&mut self, range: &Range) {
        for x in range.0 .0..=range.1 .0 {
            for y in range.0 .1..=range.1 .1 {
                self.toggle(&Coord(x, y))
            }
        }
    }

    fn turn_on(&mut self, coord: &Coord) {
        self.light_map.entry(*coord).and_modify(|entry| *entry += 1);
    }

    fn turn_off(&mut self, coord: &Coord) {
        self.light_map
            .entry(*coord)
            .and_modify(|entry| *entry = max(*entry - 1, 0));
    }

    fn toggle(&mut self, coord: &Coord) {
        self.light_map.entry(*coord).and_modify(|entry| *entry += 2);
    }

    pub fn count(&self) -> i32 {
        self.light_map.values().sum()
    }
}
