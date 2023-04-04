use std::str::FromStr;

use clap::{command, Arg};

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

struct Cookie {
    score: i32,
    calories: i32,
}

impl Cookie {
    fn new(ingredients: &[Ingredient], sum: &[i32]) -> Self {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;

        for (ingredient, teaspoons) in ingredients.iter().zip(sum.iter()) {
            capacity += ingredient.capacity * teaspoons;
            durability += ingredient.durability * teaspoons;
            flavor += ingredient.flavor * teaspoons;
            texture += ingredient.texture * teaspoons;
            calories += ingredient.calories * teaspoons;
        }

        capacity = capacity.max(0);
        durability = durability.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);

        let score = capacity * durability * flavor * texture;

        Self { score, calories }
    }
}

impl FromStr for Ingredient {
    type Err = nom::error::Error<String>;

    fn from_str<'a>(input: &'a str) -> Result<Self, Self::Err> {
        use nom::{
            bytes::complete::tag,
            character::complete::{alpha1, i32},
            combinator::all_consuming,
            Finish,
        };

        let parser = |input: &'a str| {
            let (input, _) = alpha1(input)?;
            let (input, _) = tag(": capacity ")(input)?;
            let (input, capacity) = i32(input)?;
            let (input, _) = tag(", durability ")(input)?;
            let (input, durability) = i32(input)?;
            let (input, _) = tag(", flavor ")(input)?;
            let (input, flavor) = i32(input)?;
            let (input, _) = tag(", texture ")(input)?;
            let (input, texture) = i32(input)?;
            let (input, _) = tag(", calories ")(input)?;
            let (input, calories) = i32(input)?;

            Ok((
                input,
                Self {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            ))
        };

        all_consuming(parser)(input)
            .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
            .finish()
            .map(|(_, result)| result)
    }
}

fn ways_to_sum(total: i32, num: i32) -> Vec<Vec<i32>> {
    if num == 1 {
        return vec![vec![total]];
    }

    let mut result = Vec::new();
    for i in 0..=total {
        for mut v in ways_to_sum(total - i, num - 1) {
            v.push(i);
            result.push(v);
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

    let ingredients = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .trim()
        .lines()
        .map(|line| {
            line.parse::<Ingredient>()
                .expect("Error parsing ingredient")
        })
        .collect::<Vec<_>>();

    let sums = ways_to_sum(100, ingredients.len() as i32);

    let cookies = sums
        .iter()
        .map(|sum| Cookie::new(&ingredients, sum))
        .collect::<Vec<_>>();

    let result1 = cookies
        .iter()
        .map(|cookie| cookie.score)
        .max()
        .expect("Error getting max score");

    println!("Result 1: {}", result1);

    let result2 = cookies
        .iter()
        .filter(|cookie| cookie.calories == 500)
        .map(|cookie| cookie.score)
        .max()
        .expect("Error getting max score");

    println!("Result 2: {}", result2);
}
