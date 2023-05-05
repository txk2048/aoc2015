use clap::{command, Arg};
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::all_consuming,
    sequence::{preceded, separated_pair, terminated},
    Finish,
};

fn parse_input(s: &str) -> Result<(u64, u64), nom::error::Error<String>> {
    all_consuming(preceded(
        tag("To continue, please consult the code grid in the manual.  Enter the code at row "),
        terminated(
            separated_pair(
                nom::character::complete::u64,
                tag(", column "),
                nom::character::complete::u64,
            ),
            char('.'),
        ),
    ))(s)
    .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
    .finish()
    .map(|(_, (row, column))| (row, column))
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let target =
        parse_input(&std::fs::read_to_string(&input_file).expect("Error reading input file"))
            .expect("Error parsing input");

    let mut value: u64 = 20151125;
    let mut row = 1;
    let mut column = 1;

    while (row, column) != target {
        if row == 1 {
            row = column + 1;
            column = 1;
        } else {
            row -= 1;
            column += 1;
        }
        value = (value * 252533) % 33554393;
    }

    println!("Part 1: {}", value);
}
