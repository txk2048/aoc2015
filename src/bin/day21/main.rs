use std::cmp;

use clap::{command, Arg};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Item {
    name: &'static str,
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Clone, Debug, Deserialize)]
struct Entity {
    hp: u32,
    damage: u32,
    armor: u32,
}

const WEAPONS_DATA: &str = include_str!("weapons.json");
const ARMOR_DATA: &str = include_str!("armor.json");
const RINGS_DATA: &str = include_str!("rings.json");

const PLAYER_HP: u32 = 100;

fn permutations(weapons: &[Item], armor: &[Item], rings: &[Item]) -> Vec<Vec<Item>> {
    let mut result = Vec::new();

    for weapon in weapons {
        for armor in armor {
            // no rings
            result.push(vec![weapon.clone(), armor.clone()]);

            for ring1 in rings {
                // one ring
                result.push(vec![weapon.clone(), armor.clone(), ring1.clone()]);

                for ring2 in rings {
                    if ring1.name == ring2.name {
                        continue;
                    }

                    result.push(vec![
                        weapon.clone(),
                        armor.clone(),
                        ring1.clone(),
                        ring2.clone(),
                    ]);
                }
            }
        }
    }

    result
}

fn fight(player: Entity, boss: Entity) -> bool {
    let mut player = player.clone();
    let mut boss = boss.clone();

    loop {
        // attack does min damage of 1
        let boss_damage = cmp::max(1, boss.damage.saturating_sub(player.armor));
        let player_damage = cmp::max(1, player.damage.saturating_sub(boss.armor));

        boss.hp = boss.hp.saturating_sub(player_damage);

        if boss.hp == 0 {
            return true;
        }

        player.hp = player.hp.saturating_sub(boss_damage);

        if player.hp == 0 {
            return false;
        }
    }
}

fn part1(weapons: &[Item], armor: &[Item], rings: &[Item], boss: Entity) -> Option<u32> {
    permutations(weapons, armor, rings)
        .iter()
        .filter_map(|items| {
            let total_damage = items.iter().map(|i| i.damage).sum();
            let total_armor = items.iter().map(|i| i.armor).sum();
            let total_cost: u32 = items.iter().map(|i| i.cost).sum();

            let player = Entity {
                hp: PLAYER_HP,
                damage: total_damage,
                armor: total_armor,
            };

            let player_wins = fight(player, boss.clone());

            if player_wins {
                Some(total_cost)
            } else {
                None
            }
        })
        .min()
}

fn part2(weapons: &[Item], armor: &[Item], rings: &[Item], boss: Entity) -> Option<u32> {
    permutations(weapons, armor, rings)
        .iter()
        .filter_map(|items| {
            let total_damage = items.iter().map(|i| i.damage).sum();
            let total_armor = items.iter().map(|i| i.armor).sum();
            let total_cost: u32 = items.iter().map(|i| i.cost).sum();

            let player = Entity {
                hp: PLAYER_HP,
                damage: total_damage,
                armor: total_armor,
            };

            let player_wins = fight(player, boss.clone());

            if !player_wins {
                Some(total_cost)
            } else {
                None
            }
        })
        .max()
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let input = std::fs::read_to_string(input_file).expect("Error reading input");

    let weapons: Vec<Item> = serde_json::from_str(WEAPONS_DATA).unwrap();
    let armor: Vec<Item> = serde_json::from_str(ARMOR_DATA).unwrap();
    let rings: Vec<Item> = serde_json::from_str(RINGS_DATA).unwrap();

    let boss: Entity = serde_json::from_str(&input).expect("Error parsing input");

    let result1 = part1(&weapons, &armor, &rings, boss.clone()).unwrap();
    let result2 = part2(&weapons, &armor, &rings, boss.clone()).unwrap();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
