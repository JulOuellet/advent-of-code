fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let reports = parse_input(&input);

    println!("Part one: {}", part_one(reports));

    Ok(())
}


pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}


pub fn part_one(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;

    reports.iter().for_each(|report| {
        if report.len() == 0 {
            return;
        }

        let is_increasing = report.windows(2).all(|w| w[0] < w[1]);
        let is_decreasing = report.windows(2).all(|w| w[0] > w[1]);
        let is_bound = report.windows(2).all(|w| (w[0] - w[1]).abs() <= 3 && (w[0] - w[1]).abs() > 0);

        if is_increasing && is_bound {
            safe_reports += 1;
        } else if is_decreasing && is_bound {
            safe_reports += 1;
        }
    });

    safe_reports
}

