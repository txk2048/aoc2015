mod parser;

use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use clap::{command, Arg};
use itertools::Itertools;

struct Distance {
    from: String,
    to: String,
    distance: u32,
}

fn parse_input(input_file: &str) -> (HashSet<String>, HashMap<String, HashMap<String, u32>>) {
    let input = std::fs::read_to_string(input_file).expect("Error reading input file");

    let mut result = HashMap::new();
    let mut destinations = HashSet::new();

    for line in input.trim().lines() {
        let distance: Distance = line.parse().expect("Error parsing distance");

        // sort alphabetically
        let first = cmp::min(&distance.from, &distance.to);
        let second = cmp::max(&distance.from, &distance.to);

        destinations.insert(first.to_string());
        destinations.insert(second.to_string());

        result
            .entry(first.to_string())
            .or_insert_with(HashMap::new)
            .insert(second.to_string(), distance.distance);
    }

    (destinations, result)
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let (destinations, distances) = parse_input(&input_file);

    let (result1, result2) = destinations
        .iter()
        .permutations(destinations.len())
        .map(|route| {
            route.iter().tuple_windows().fold(0, |acc, (from, to)| {
                let first = cmp::min(from, to);
                let second = cmp::max(from, to);

                acc + distances[*first][*second]
            })
        })
        .minmax()
        .into_option()
        .unwrap();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
