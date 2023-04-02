use clap::{command, Arg};

fn is_nice1(s: &str) -> bool {
    let vowel_count = s.chars().filter(|c| "aeiou".contains(*c)).count();
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    let has_bad = ["ab", "cd", "pq", "xy"].iter().any(|b| s.contains(b));

    vowel_count >= 3 && has_double && !has_bad
}

fn is_nice2(s: &str) -> bool {
    let has_double = s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b);

    let pairs = s
        .chars()
        .zip(s.chars().skip(1))
        .map(|(a, b)| format!("{}{}", a, b))
        .enumerate();

    let has_repeat = pairs.into_iter().any(|(index, pair)| {
        let last = s.rfind(&pair).unwrap();
        last > index + 1 // they are not overlapping
    });

    has_double && has_repeat
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
        .map(|s| s.to_string())
        .collect();

    let nice1 = strings.iter().filter(|s| is_nice1(s)).count();
    let nice2 = strings.iter().filter(|s| is_nice2(s)).count();

    println!("Part 1: {}", nice1);
    println!("Part 2: {}", nice2);
}
