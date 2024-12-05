fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (left, right) = parse_input(&input);

    println!("Part 1: {}", part_one(&left, &right));
    println!("Part 2: {}", part_two(&left, &right));

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

pub fn part_one(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut sorted_left = left.clone();
    let mut sorted_right = right.clone();

    sorted_left.sort();
    sorted_right.sort();

    sorted_left.iter()
        .zip(sorted_right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn part_two(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut scores = Vec::new();

    for &item in left {
        let count = right.iter().filter(|&&x| x == item).count();
        scores.push((item, count));
    }

    scores.iter()
        .map(|&(item, count)| item * count as i32)
        .sum()
}

