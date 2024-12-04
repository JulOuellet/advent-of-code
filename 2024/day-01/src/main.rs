

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    println!("Part 1: {}", input);

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
