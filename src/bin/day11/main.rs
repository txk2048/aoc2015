use clap::{command, Arg};
use itertools::Itertools;

fn increment_str(pw: &mut [char]) {
    if pw.len() == 0 {
        return;
    }

    let len = pw.len();
    let last = pw.iter().last().unwrap().clone();
    let rem = &mut pw[..len - 1];

    if last == 'z' {
        increment_str(rem);
        pw[pw.len() - 1] = 'a';
    } else {
        pw[pw.len() - 1] = (last as u8 + 1) as char;
    }
}

fn is_valid(pw: &[char]) -> bool {
    let has_straight = pw.iter().tuple_windows().any(|(a, b, c)| {
        // find a straight of 3 letters
        let a = *a as u8;
        let b = *b as u8;
        let c = *c as u8;

        a + 1 == b && b + 1 == c
    });

    let has_no_bad_letters = pw.iter().all(|c| *c != 'i' && *c != 'o' && *c != 'l');

    let has_two_pairs = pw
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a == b)
        .unique()
        .count()
        >= 2;

    has_straight && has_no_bad_letters && has_two_pairs
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let mut password: Vec<char> = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .trim()
        .chars()
        .collect();

    while !is_valid(&password) {
        increment_str(&mut password);
    }

    println!("Part 1: {}", password.iter().collect::<String>());

    // password is valid, so increment it to (potentilally) invalidate
    increment_str(&mut password);

    while !is_valid(&password) {
        increment_str(&mut password);
    }

    println!("Part 2: {}", password.iter().collect::<String>());
}
