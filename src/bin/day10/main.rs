use clap::{command, Arg};

fn apply(value: &str) -> String {
    let mut result = String::new();

    let mut chars = value.chars().peekable();
    let mut count = 0;

    while let Some(c) = chars.next() {
        count += 1;

        if chars.peek() != Some(&c) {
            result.push_str(&count.to_string());
            result.push(c);
            count = 0;
        }
    }

    result
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let mut value = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .trim()
        .to_string();

    for _ in 0..40 {
        value = apply(&value);
    }

    println!("Part 1: {}", value.len());

    for _ in 0..10 {
        value = apply(&value);
    }

    println!("Part 2: {}", value.len());
}
