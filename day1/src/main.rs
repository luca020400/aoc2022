#![warn(clippy::pedantic)]

use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = fs::read_to_string("input.txt")?
        .split("\n\n")
        .map(|elf| elf.lines().map(|calories| calories.parse::<u32>()).sum())
        .collect::<Result<Vec<u32>, _>>()?;

    data.sort_unstable();

    let part_one = match data.last() {
        Some(x) => Ok(x),
        None => Err("Must have at least one element"),
    }?;
    println!("Part one: {part_one}");

    let part_two = data[data.len() - 3..].iter().sum::<u32>();
    println!("Path two: {part_two}");

    Ok(())
}
