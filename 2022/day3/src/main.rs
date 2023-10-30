use std::{
    fs::File,
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);

    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .for_each(|line| {
            // TODO solve problem
        });
    Ok(())
}

