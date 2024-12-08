use regex::Regex;
use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

pub fn part_one(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let multiplications: Vec<(i32, i32)> = regex.captures_iter(input)
        .filter_map(|cap| {
            match (cap[1].trim().parse(), cap[2].trim().parse()) {
                (Ok(x), Ok(y)) => Some((x, y)),
                _ => None
            }
        })
    .collect();

    multiplications.iter().map(|(x, y)| x * y).sum()
}

pub fn part_two(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    let mut is_enabled = true;
    let mut total = 0;

    for (_, instruction) in instructions {
        match instruction.as_str() {
            "do()" => is_enabled = true,
            "don't()" => is_enabled = false,
            _ => {
                if is_enabled && instruction.starts_with("mul(") {
                    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                    
                    if let Some(captures) = regex.captures(&instruction) {
                        if let (Ok(x), Ok(y)) = (
                            captures[1].parse::<i32>(),
                            captures[2].parse::<i32>(),
                        ) {
                            total += x * y;
                        }
                    }
                }
            }
        }
    }

    total
}

pub fn parse_instructions(input: &str) -> BTreeMap<usize, String> {
    Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)")
        .unwrap()
        .find_iter(input)
        .enumerate()
        .map(|(i, m)| (i, m.as_str().to_string()))
        .collect()
}

