use std::path::Iter;

use itertools::iproduct;

fn main() {
    let me = Fighter {
        hitpoints: 8,
        damage: 5,
        armor: 5,
    };
    let monster = Fighter {
        hitpoints: 12,
        damage: 7,
        armor: 2,
    };
    for item_set in item_sets_iter {
        println!("{}"item_set);
    }
}

fn find_item_set(budget: i32) {
    // todo
}

fn item_sets_iter() -> itertools::ConsTuples<
    itertools::Product<
        itertools::ConsTuples<
            itertools::Product<
                itertools::Product<
                    std::slice::Iter<'static, Item>,
                    std::slice::Iter<'static, Item>,
                >,
                std::slice::Iter<'static, Item>,
            >,
            ((&'static Item, &'static Item), &'static Item),
        >,
        std::slice::Iter<'static, Item>,
    >,
    ((&'static Item, &'static Item, &'static Item), &'static Item),
> {
    iproduct!(WEAPONS.iter(), ARMORS.iter(), RINGS.iter(), RINGS.iter())
}

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

const ARMORS: [Item; 5] = [
    Item::new(13, 0, 1),
    Item::new(31, 0, 2),
    Item::new(53, 0, 3),
    Item::new(75, 0, 4),
    Item::new(102, 0, 5),
];

const RINGS: [Item; 6] = [
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
