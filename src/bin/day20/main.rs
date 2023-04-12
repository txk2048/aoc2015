use clap::{command, Arg};

fn sum_divisors(n: u32) -> u32 {
    let mut sum = 0;

    let stop = (n as f64).sqrt() as u32;
    for i in 1..=stop {
        if n % i == 0 {
            sum += i;

            let j = n / i;
            if j != i {
                sum += j;
            }
        }
    }

    sum
}

fn part1(num: u32) -> Option<u32> {
    for house_num in 1.. {
        let presents = sum_divisors(house_num) * 10;

        if presents >= num {
            return Some(house_num);
        }
    }

    None
}

fn part2(num: u32) -> Option<u32> {
    for house_num in 1.. {
        let mut presents = 0;

        // house_num == elf_num * k
        // for some k where 1 <= k <= 50
        // by trying all possible k, we can find the corresponding elf_num
        for k in 1..=50 {
            if house_num % k != 0 {
                continue;
            }

            let elf_num = house_num / k;
            presents += elf_num * 11;
        }

        if presents >= num {
            return Some(house_num);
        }
    }

    None
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let num: u32 = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .parse()
        .expect("Error parsing input file");

    println!("Part 1: {}", part1(num).expect("No solution found"));
    println!("Part 2: {}", part2(num).expect("No solution found"));
}
