use clap::{command, Arg};
use serde_json::Value;

fn sum(root: &Value, ignore_red: bool) -> i64 {
    match root {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| sum(v, ignore_red)).sum(),
        Value::Object(o) => {
            if ignore_red && o.values().any(|v| v == "red") {
                0
            } else {
                o.values().map(|v| sum(v, ignore_red)).sum()
            }
        }
        _ => 0,
    }
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let data: Value = serde_json::from_reader(
        std::fs::File::open(input_file).expect("Could not open input file"),
    )
    .expect("Error parsing input file");

    println!("Part 1: {}", sum(&data, false));
    println!("Part 2: {}", sum(&data, true));
}
