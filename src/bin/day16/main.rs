mod parser;

use std::collections::HashMap;

use clap::{command, Arg};

struct Aunt {
    id: u32,
    attributes: HashMap<String, u32>,
}

// find the id of the aunt that matches the target attributes
fn part1(aunts: &[Aunt], target_attributes: &HashMap<&str, u32>) -> u32 {
    let aunt = aunts
        .iter()
        .find(|aunt| {
            aunt.attributes.iter().all(|(k, v)| {
                let target_value = target_attributes.get(k.as_str()).unwrap();
                v == target_value
            })
        })
        .expect("No aunt found");

    aunt.id
}

fn part2(aunts: &[Aunt], target_attributes: &HashMap<&str, u32>) -> u32 {
    let aunt = aunts
        .iter()
        .find(|aunt| {
            aunt.attributes.iter().all(|(k, v)| {
                let target_value = target_attributes.get(k.as_str()).unwrap();
                match k.as_str() {
                    "cats" | "trees" => v > target_value,
                    "pomeranians" | "goldfish" => v < target_value,
                    _ => v == target_value,
                }
            })
        })
        .expect("No aunt found");

    aunt.id
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let aunts = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .trim()
        .lines()
        .map(|line| line.parse::<Aunt>().expect("Error parsing aunt"))
        .collect::<Vec<_>>();

    let target_attributes: HashMap<&str, u32> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    println!("Part 1: {}", part1(&aunts, &target_attributes));
    println!("Part 2: {}", part2(&aunts, &target_attributes));
}
