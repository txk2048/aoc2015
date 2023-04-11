use bitvec::vec::BitVec;
use clap::{command, Arg};

const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

fn get_neighbors(grid: &BitVec, x: i32, y: i32) -> i32 {
    let mut neighbors = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let ny = y + dy;
            let nx = x + dx;

            // bounds check
            if ny < 0 || ny >= HEIGHT || nx < 0 || nx >= WIDTH {
                continue;
            }

            let index: usize = (ny * WIDTH + nx).try_into().unwrap();

            if grid[index] {
                neighbors += 1;
            }
        }
    }
    neighbors
}

fn simulate(grid: &BitVec, next: &mut BitVec) {
    next.resize(grid.len(), false);

    assert_eq!(grid.len(), next.len());

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let neighbors = get_neighbors(grid, x, y);

            let index = (y * WIDTH + x).try_into().unwrap();

            if grid[index] {
                next.set(index, neighbors == 2 || neighbors == 3);
            } else {
                next.set(index, neighbors == 3);
            }
        }
    }
}

fn part1(grid: &BitVec) -> usize {
    let mut grid = grid.clone();
    let mut next = BitVec::new();

    for _ in 0..100 {
        simulate(&grid, &mut next);
        std::mem::swap(&mut grid, &mut next);
    }

    grid.count_ones()
}

fn part2(grid: &BitVec) -> usize {
    let mut grid = grid.clone();
    let mut next = BitVec::new();

    let corners = [
        (0, 0),
        (0, HEIGHT - 1),
        (WIDTH - 1, 0),
        (WIDTH - 1, HEIGHT - 1),
    ];

    // force corners on
    for (x, y) in corners {
        let index: usize = (y * WIDTH + x).try_into().unwrap();
        grid.set(index, true);
    }

    for _ in 0..100 {
        simulate(&grid, &mut next);
        std::mem::swap(&mut grid, &mut next);

        // force corners on
        for (x, y) in corners {
            let index: usize = (y * WIDTH + x).try_into().unwrap();
            grid.set(index, true);
        }
    }

    grid.count_ones()
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let grid: BitVec = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .trim()
        .lines()
        .flat_map(|line| line.chars().map(|c| c == '#'))
        .collect();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
