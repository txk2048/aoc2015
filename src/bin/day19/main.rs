mod parser;

use clap::{command, Arg};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Replacement {
    from: String,
    to: String,
}

fn all_replacements<'a>(
    molecule: &'a str,
    replacement: &'a Replacement,
) -> impl Iterator<Item = String> + 'a {
    molecule.match_indices(&replacement.from).map(|(i, _)| {
        let mut new_molecule = molecule.to_string();
        new_molecule.replace_range(i..i + replacement.from.len(), &replacement.to);
        new_molecule
    })
}

fn part1(molecule: &str, replacements: &[Replacement]) -> usize {
    replacements
        .iter()
        .flat_map(|r| all_replacements(molecule, r))
        .unique()
        .count()
}

// see https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/
fn part2(molecule: &str) -> usize {
    // each element begins with a capital letter
    let total = molecule.matches(char::is_uppercase).count();

    let ar_rn = molecule.matches("Ar").count() + molecule.matches("Rn").count();
    let y = molecule.matches("Y").count();

    total - ar_rn - (2 * y) - 1
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let input = std::fs::read_to_string(input_file).expect("Error reading input file");

    let (replacements, molecule) = parser::parse_input(&input).expect("Error parsing input");

    println!("Part 1: {}", part1(&molecule, &replacements));
    println!("Part 2: {}", part2(&molecule));
}
