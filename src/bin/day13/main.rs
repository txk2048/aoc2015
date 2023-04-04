use std::collections::{HashMap, HashSet};

use clap::{command, Arg};
use itertools::Itertools;

fn parse_line<'a>(input: &'a str) -> Result<(String, String, i32), nom::error::Error<String>> {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::char,
        character::complete::i32, combinator::all_consuming, Finish,
    };

    let parser = |input: &'a str| {
        let (input, person1) = alpha1(input)?;
        let (input, _) = tag(" would ")(input)?;
        let (input, gain_or_lose) = alt((tag("gain"), tag("lose")))(input)?;
        let (input, _) = char(' ')(input)?;
        let (input, happiness) = i32(input)?;
        let (input, _) = tag(" happiness units by sitting next to ")(input)?;
        let (input, person2) = alpha1(input)?;
        let (input, _) = char('.')(input)?;

        let val = match gain_or_lose {
            "gain" => (person1.to_string(), person2.to_string(), happiness),
            "lose" => (person1.to_string(), person2.to_string(), -happiness),
            _ => unreachable!(),
        };

        Ok((input, val))
    };

    all_consuming(parser)(input)
        .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
        .finish()
        .map(|(_, v)| v)
}

fn parse_input(input_file: &str) -> HashMap<(String, String), i32> {
    let input = std::fs::read_to_string(input_file).expect("Error reading input file");

    input
        .trim()
        .lines()
        .map(|line| parse_line(line).expect("Error parsing line"))
        .map(|(p1, p2, h)| ((p1, p2), h))
        .collect()
}

fn compute_happiness(happiness: &HashMap<(String, String), i32>, arrangement: &[&String]) -> i32 {
    arrangement
        .iter()
        .circular_tuple_windows()
        .fold(0, |acc, (p1, p2)| {
            acc + happiness[&(p1.to_string(), p2.to_string())]
                + happiness[&(p2.to_string(), p1.to_string())]
        })
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let mut happiness = parse_input(input_file);
    let people = happiness
        .keys()
        .map(|(p1, _)| p1.to_string())
        .collect::<HashSet<String>>();

    let result1 = people
        .iter()
        .permutations(people.len())
        .map(|arrangement| compute_happiness(&happiness, &arrangement))
        .max()
        .unwrap();

    println!("Result 1: {}", result1);

    let me = "Me".to_string();
    for person in people.iter() {
        happiness.insert((me.clone(), person.clone()), 0);
        happiness.insert((person.clone(), me.clone()), 0);
    }

    let result2 = people
        .iter()
        .chain(std::iter::once(&me))
        .permutations(people.len() + 1)
        .map(|arrangement| compute_happiness(&happiness, &arrangement))
        .max()
        .unwrap();

    println!("Result 2: {}", result2);
}
