use clap::{command, Arg};

fn solve(key: &str) -> (Option<usize>, Option<usize>) {
    let mut part1 = None;
    let mut part2 = None;

    for n in 0usize..usize::MAX {
        let hash = md5::compute(format!("{}{}", key, n));

        // Check if the first 5 bytes are 0
        if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0 {
            part1.get_or_insert(n);
        }

        // Check if the first 6 bytes are 0
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            part2.get_or_insert(n);
        }

        if part1.is_some() && part2.is_some() {
            break;
        }
    }

    (part1, part2)
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let key = std::fs::read_to_string(&input_file)
        .expect("Error reading input")
        .trim()
        .to_string();

    let (part1, part2) = solve(&key);
    match part1 {
        Some(n) => println!("Part 1: {}", n),
        None => println!("Part 1: No solution found"),
    };

    match part2 {
        Some(n) => println!("Part 2: {}", n),
        None => println!("Part 2: No solution found"),
    };
}
