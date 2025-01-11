use std::slice::Iter;

use itertools::{iproduct, Itertools};

fn main() {
    let boss = Fighter {
        hitpoints: 103,
        damage: 9,
        armor: 2,
    };
    let mut lowest_price = i32::MAX;
    let mut max_price = i32::MIN;

    for item_set in item_sets_iter() {
        let (weapon, armor, ring_set) = item_set;
        let (ring1, ring2) = (ring_set[0], ring_set[1]);
        let total_price = weapon.cost + armor.cost + ring1.cost + ring2.cost;
        let me = Fighter {
            hitpoints: 100,
            damage: weapon.damage + armor.damage + ring1.damage + ring2.damage,
            armor: weapon.armor + armor.armor + ring1.armor + ring2.armor,
        };

        let win = me.wins(&boss);

        if win && total_price < lowest_price {
            lowest_price = total_price;
        }
        if !win && total_price > max_price {
            max_price = total_price
        }
        println!("With the price of {}, win is {}", total_price, win);
    }
    println!("Overall, cheapest win is with price {}", lowest_price);
    println!("Overall, most expensive loss is with price {}", max_price);
}

fn item_sets_iter() -> itertools::ConsTuples<
    itertools::Product<
        itertools::Product<Iter<'static, Item>, Iter<'static, Item>>,
        itertools::Combinations<Iter<'static, Item>>,
    >,
    ((&'static Item, &'static Item), Vec<&'static Item>),
> {
    let ring_sets = RINGS.iter().combinations(2);
    iproduct!(WEAPONS.iter(), ARMORS.iter(), ring_sets)
}

#[derive(Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    const fn new(cost: i32, damage: i32, armor: i32) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item::new(8, 4, 0),
    Item::new(10, 5, 0),
    Item::new(25, 6, 0),
    Item::new(40, 7, 0),
    Item::new(74, 8, 0),
];

const ARMORS: [Item; 6] = [
    Item::new(0, 0, 0),
    Item::new(13, 0, 1),
    Item::new(31, 0, 2),
    Item::new(53, 0, 3),
    Item::new(75, 0, 4),
    Item::new(102, 0, 5),
];

const RINGS: [Item; 8] = [
    Item::new(0, 0, 0),
    Item::new(0, 0, 0),
    Item::new(25, 1, 0),
    Item::new(50, 2, 0),
    Item::new(100, 3, 0),
    Item::new(20, 0, 1),
    Item::new(40, 0, 2),
    Item::new(80, 0, 3),
];

#[derive(Debug)]
struct Fighter {
    hitpoints: i32,
    damage: i32,
    armor: i32,
}

impl Fighter {
    pub fn wins(&self, other: &Fighter) -> bool {
        let mut self_hitpoints = self.hitpoints;
        let mut other_hitpoints = other.hitpoints;

        loop {
            other_hitpoints -= calculate_hit(self.damage, other.armor);
            if other_hitpoints <= 0 {
                return true;
            }
            self_hitpoints -= calculate_hit(other.damage, self.armor);
            if self_hitpoints <= 0 {
                return false;
            }
        }
    }
}

fn calculate_hit(damage: i32, armor: i32) -> i32 {
    std::cmp::max(damage - armor, 1)
}
