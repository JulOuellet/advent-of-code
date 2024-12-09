fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = parse_input(&input);

    println!("Part One: {}", part_one(&grid));
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line
             .chars()
             .filter(|&char| matches!(char, 'X' | 'M' | 'A' | 'S'))
             .collect())
        .collect()
}

pub fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let mut xmas_count = 0;

    // Loop on each char
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &char) in row.iter().enumerate() {
            if char == 'X' {
                xmas_count += process_node(&grid, row_index, col_index);
            }
        }
    }

    xmas_count
}

fn process_node(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> usize {
    let directions = [
        (0, 1),  // Right
        (0, -1), // Left
        (1, 0),  // Down
        (-1, 0), // Up
        (1, 1),  // Down-Right
        (1, -1), // Down-Left
        (-1, 1), // Up-Right
        (-1, -1) // Up-Left
    ];

    let mut xmas_count = 0;

    for &(delta_row, delta_col) in &directions {
        if check_xmas(grid, row_index, col_index, delta_row, delta_col) {
            xmas_count += 1;
        }
    }

    xmas_count
}


fn check_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize, delta_row: isize, delta_col: isize) -> bool {
    let word = ['X', 'M', 'A', 'S'];
    for (i, &char) in word.iter().enumerate() {
        let new_row = row as isize + i as isize * delta_row;
        let new_col = col as isize + i as isize * delta_col;

        if new_row < 0
            || new_col < 0
            || new_row >= grid.len() as isize
            || new_col >= grid[0].len() as isize
            || grid[new_row as usize][new_col as usize] != char
        {
            return false;
        }
    }
    true
}
