use std::collections::HashSet;
use std::io::{self, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn part1(input: &str) -> Result<()> {
    let mut freq = 0;
    for line in input.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }
    writeln!(io::stdout(), "{}", freq)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for line in input.lines() {
            let change: i32 = line.parse()?;
            freq += change;
            if seen.contains(&freq) {
                writeln!(io::stdout(), "{}", freq)?;
                return Ok(());
            }
            seen.insert(freq);
        }
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input/input.txt");
    // std::fs::File::open("../input/input.txt");
    // io::stdin().read_to_end(&mut input)?;

    part1(input)?;
    part2(input)?;
    Ok(())
}
