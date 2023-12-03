use std::{
    fs::File,
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(&input);
    
    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            // TODO: solve puzzle
        });

    Ok(())
}
