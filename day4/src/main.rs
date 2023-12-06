use std::{fs::File, io::Read, path::Path, str::FromStr};

fn main() {
    let input = read_file_to_string("test.txt");

    let cards: Vec<_> = input
        .lines()
        .map(|line| {
            let line = match line.strip_suffix('\n') {
                Some(line) => line,
                None => line,
            };
            line.parse::<Card>().unwrap()
        })
        .collect();
    let card_set = CardSet { cards };

    println!("The total sum: {}", card_set.total_value());
}

struct CardSet {
    cards: Vec<Card>,
}

impl CardSet {
    fn total_value(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.cards.len() {
            sum += self.value_of_card(i);
        }
        sum
    }
    // Lazy non-dynamic implementation
    fn value_of_card(&self, index: usize) -> usize {
        if index > self.cards.len() {
            return 0;
        }
        let match_count = self.cards[index].match_count();
        let mut sum = 0;
        for i in index + 1..=index + match_count {
            sum += self.value_of_card(i);
        }
        sum + 1
    }
}

#[derive(Debug)]
struct Card {
    card_id: usize,
    pub winning_numbers: Vec<usize>,
    pub numbers: Vec<usize>,
}

impl Card {
    fn match_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn value(&self) -> u32 {
        let c = self.match_count() as u32;
        match c {
            0 => 0,
            1 => 1,
            n => 2_u32.pow(n - 1),
        }
    }
}

fn numbers_from_string(s: &str) -> Vec<usize> {
    s.trim()
        .split(' ')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                return None;
            }
            Some(s.parse().expect("Should be able to parse strings"))
        })
        .collect()
}

#[derive(Debug)]
struct ParseError {}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card ").unwrap();
        let (s1, rest) = s.split_once(':').unwrap();
        let (s2, s3) = rest.split_once('|').unwrap();

        let card_id = s1.trim().parse::<usize>().unwrap();
        Ok(Card {
            card_id,
            winning_numbers: numbers_from_string(s2),
            numbers: numbers_from_string(s3),
        })
    }
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
