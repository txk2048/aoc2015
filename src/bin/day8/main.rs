use clap::{command, Arg};

fn decode_string(input: &str) -> Option<usize> {
    if input.len() < 2 {
        return None;
    }

    let mut index = 1;
    let mut count = 0;

    while index < input.len() - 1 {
        let c = input.chars().nth(index).unwrap();

        // escaped character
        if c == '\\' {
            if let Some(c2) = input.chars().nth(index + 1) {
                if c2 == '\\' || c2 == '"' {
                    // \\ or \"
                    count += 1;
                    index += 2;
                } else if c2 == 'x' {
                    // \xHH
                    count += 1;
                    index += 4;
                } else {
                    // invalid escape sequence - invalid character
                    return None;
                }
            } else {
                // invalid escape sequence - missing character
                return None;
            }
        } else {
            // normal character
            count += 1;
            index += 1;
        }
    }

    Some(count)
}

fn encode_string(input: &str) -> usize {
    2 + input.chars().fold(0, |acc, c| {
        if c == '\\' || c == '"' {
            // escaped character
            acc + 2
        } else {
            // normal character
            acc + 1
        }
    })
}

fn part1(strings: &[String]) -> Option<usize> {
    Some(
        strings
            .iter()
            .map(|s| Some(s.len() - decode_string(s)?))
            .sum::<Option<usize>>()?,
    )
}

fn part2(strings: &[String]) -> usize {
    strings.iter().map(|s| encode_string(s) - s.len()).sum()
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let strings: Vec<String> = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .trim()
        .lines()
        .map(|s| s.to_owned())
        .collect();

    match part1(&strings) {
        Some(result) => println!("Part 1: {}", result),
        None => println!("Part 1: invalid input"),
    }

    println!("Part 2: {}", part2(&strings));
}
