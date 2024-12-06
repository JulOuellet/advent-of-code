fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let reports = parse_input(&input);

    println!("Part one: {}", part_one(&reports));
    println!("Part two: {}", part_two(&reports));

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


pub fn part_one(reports: &Vec<Vec<i32>>) -> i32 {
    reports.iter().filter(|report| is_safe(report)).count() as i32
}

pub fn part_two(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .filter(|report| {
            if is_safe(report) {
                return true;
            }

            for i in 0..report.len() {
                let modified_report: Vec<i32> = report.iter()
                    .enumerate()
                    .filter(|&(index, _)| index != i)
                    .map(|(_, &val)| val)
                    .collect();

                if is_safe(&modified_report) {
                    return true;
                }
            }

            false
        })
    .count() as i32
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() == 0 {
        return false;
    }

    let is_increasing = report.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = report.windows(2).all(|w| w[0] > w[1]);
    let is_bound = report.windows(2).all(|w| (w[0] - w[1]).abs() <= 3 && (w[0] - w[1]).abs() > 0);

    if is_increasing && is_bound {
        return true;
    } else if is_decreasing && is_bound {
        return true;
    }

    false
}

