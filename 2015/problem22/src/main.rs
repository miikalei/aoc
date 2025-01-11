use itertools::{self, repeat_n, Itertools};
use std::cmp::max;

fn main() {
    let mut fight = Fight {
        player_health: 50,
        player_mana: 500,
        boss_damage: 9,
        boss_health: 51,
        effects: vec![],
    };

    // let plan = vec![
    //     &POISON,
    //     &MAGIC_MISSILE,
    //     &RECHARGE,
    //     &POISON,
    //     &MAGIC_MISSILE,
    //     &SHIELD,
    //     &MAGIC_MISSILE,
    // ];
    // fight.fight(&plan);
    // println!("Total mana cost: {}", total_mana_cost(&plan));

    let mut min_mana_win = i32::MAX;
    for seq in spell_sequence_iter().take(3000000) {
        // println!();
        // println!("Trying out a new fight with sequence: {:?}", seq);
        let mut fight = fight.clone();
        if let Some(true) = fight.fight(&seq) {
            let mana_cost = total_mana_cost(&seq);
            if mana_cost < min_mana_win {
                println!("Record, total mana cost: {}", mana_cost);
                println!("Spell sequence used: {:?}", seq);

                min_mana_win = mana_cost
            }
        }
    }
    println!("Cheapest win happened with {} mana", min_mana_win)
}

const SPELLS: [&Spell<'_>; 5] = [&MAGIC_MISSILE, &DRAIN, &SHIELD, &POISON, &RECHARGE];
fn spell_sequence_iter() -> impl Iterator<Item = std::vec::Vec<&'static Spell<'static>>> {
    (1..).flat_map(|k| repeat_n(SPELLS.into_iter(), k).multi_cartesian_product())
}

fn total_mana_cost(spells: &[&Spell]) -> i32 {
    spells.iter().fold(0, |acc, s| acc + s.mana_cost)
}

const SHIELD_EFFECT: Effect = Effect {
    name: "Shield effect",
    turns_left: 6,
    armor_increase: 7,
    damage_per_turn: 0,
    mana_recharge: 0,
};
const POISON_EFFECT: Effect = Effect {
    name: "Poison effect",
    turns_left: 6,
    armor_increase: 0,
    damage_per_turn: 3,
    mana_recharge: 0,
};
const RECHARGE_EFFECT: Effect = Effect {
    name: "Recharge effect",
    turns_left: 5,
    armor_increase: 0,
    damage_per_turn: 0,
    mana_recharge: 101,
};

const MAGIC_MISSILE: Spell = Spell {
    name: "magic missile",
    mana_cost: 53,
    damage: 4,
    heal: 0,
    effect: None,
};
const DRAIN: Spell = Spell {
    name: "drain",
    mana_cost: 73,
    damage: 2,
    heal: 2,
    effect: None,
};
const SHIELD: Spell = Spell {
    name: "shield",
    mana_cost: 113,
    damage: 0,
    heal: 0,
    effect: Some(&SHIELD_EFFECT),
};
const POISON: Spell = Spell {
    name: "poison",
    mana_cost: 173,
    damage: 0,
    heal: 0,
    effect: Some(&POISON_EFFECT),
};
const RECHARGE: Spell = Spell {
    name: "recharge",
    mana_cost: 229,
    damage: 0,
    heal: 0,
    effect: Some(&RECHARGE_EFFECT),
};

#[derive(Clone, Debug)]
struct Effect {
    name: &'static str,
    turns_left: i32,

    armor_increase: i32,
    damage_per_turn: i32,
    mana_recharge: i32,
}

#[derive(Debug)]
struct Spell<'a> {
    name: &'a str,
    mana_cost: i32,
    damage: i32,
    heal: i32,
    effect: Option<&'a Effect>,
}

#[derive(Debug, Clone)]
struct Fight {
    boss_health: i32,
    boss_damage: i32,

    player_health: i32,
    player_mana: i32,
    effects: Vec<Effect>,
}

impl Fight {
    fn player_armor(&self) -> i32 {
        self.effects.iter().fold(0, |acc, x| acc + x.armor_increase)
    }

    fn start_turn_damage(&self) -> i32 {
        self.effects
            .iter()
            .fold(0, |acc, x| acc + x.damage_per_turn)
    }

    fn start_turn_mana_recharge(&self) -> i32 {
        self.effects.iter().fold(0, |acc, x| acc + x.mana_recharge)
    }

    fn shorten_effects(&mut self) {
        self.effects.retain_mut(|effect| {
            effect.turns_left -= 1;
            effect.turns_left > 0
        });
    }

    fn start_turn(&mut self) {
        let start_turn_dmg = self.start_turn_damage();
        if start_turn_dmg > 0 {
            // println!("Boss takes {} start of turn damage", start_turn_dmg);
            self.boss_health -= self.start_turn_damage();
        }
        let mana_gain = self.start_turn_mana_recharge();
        if mana_gain > 0 {
            // println!("Player gains {} start of turn mana", mana_gain);
            self.player_mana += mana_gain;
        }
        self.shorten_effects();
    }

    fn enemy_turn(&mut self) {
        let damage = max(self.boss_damage - self.player_armor(), 1);
        // println!("Enemy deals {} damage!", damage);
        self.player_health -= damage;
    }

    fn player_turn(&mut self, spell: &Spell) -> Option<bool> {
        // Check that effects not around
        if let Some(effect) = spell.effect {
            if self.effects.iter().any(|e| e.name == effect.name) {
                return Some(false);
            }
        }

        self.player_mana -= spell.mana_cost;
        // Check that mana does not go negative
        if self.player_mana < 0 {
            return Some(false);
        }

        if spell.heal > 0 {
            // println!("Player heals {} hitpoints", spell.heal);
            self.player_health += spell.heal;
        }
        if spell.damage > 0 {
            // println!("Player deals {} damage", spell.damage);
            self.boss_health -= spell.damage;
        }
        if let Some(effect) = spell.effect {
            self.effects.push(effect.clone());
        }
        None
    }

    fn player_won(&self) -> Option<bool> {
        if self.boss_health <= 0 {
            return Some(true);
        }
        if self.player_health <= 0 {
            return Some(false);
        }
        None
    }

    fn print_situation(&self) {
        // println!("{:?}", self)
    }

    pub fn fight(&mut self, spells: &Vec<&Spell>) -> Option<bool> {
        for spell in spells {
            // println!("New turn starts.");
            self.start_turn();
            self.print_situation();
            if let Some(win) = self.player_won() {
                // println!("Match ends! Win: {}", win);
                return Some(win);
            }
            // println!("Player has their turn.");
            if let Some(false) = self.player_turn(spell) {
                // println!("Player loses.");
                return Some(false); // Illegal sequence
            }
            self.print_situation();
            if let Some(win) = self.player_won() {
                // println!("Match ends! Win: {}", win);
                return Some(win);
            }
            // println!("New turn starts.");
            self.start_turn();
            self.print_situation();
            if let Some(win) = self.player_won() {
                // println!("Match ends! Win: {}", win);
                return Some(win);
            }
            // println!("Enemy has their turn.");
            self.enemy_turn();
            self.print_situation();
            if let Some(win) = self.player_won() {
                // println!("Match ends! Win: {}", win);
                return Some(win);
            }
        }
        // println!("Out of moves, match ends.");
        None
    }
}
