fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (left, right) = parse_input(&input);

    println!("Part 1: {}", part_one(left, right));

    Ok(())
}

pub fn parse_input(input: &str) -> (Vec<i32>,Vec<i32>) {
    input
        .lines()
        .filter_map(|line| {
            let nums: Result<Vec<i32>, _> = line
                .split_whitespace()
                .map(|num| num.parse())
                .collect();
            
            nums.ok()
                .filter(|v| v.len() == 2)
                .map(|v| (v[0], v[1]))
        })
        .unzip()
}

fn part_one(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}
