use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part one: {}", part_one(&input));
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
    0
}
