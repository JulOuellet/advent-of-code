use std::{
    fs::File, 
    io::{BufRead, BufReader}
};

fn main() -> std::io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // TODO: iterate on reader and solve puzzle

    Ok(())
}

