use clap::{command, Arg};

mod game;
mod parser;

#[derive(Copy, Clone)]
struct Boss {
    hp: i32,
    damage: i32,
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let boss = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .trim()
        .parse::<Boss>()
        .expect("Error parsing input file");

    println!("Part 1: {}", game::run(boss, false));
    println!("Part 2: {}", game::run(boss, true));
}
