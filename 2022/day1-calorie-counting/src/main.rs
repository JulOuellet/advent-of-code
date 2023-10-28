mod part_one;

use std::{fs::File, io::BufReader};
use crate::part_one::part_one_solution;

fn main() -> std::io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    part_one_solution(reader)
}

