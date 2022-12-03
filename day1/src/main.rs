#![warn(clippy::pedantic)]

use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut data: Vec<u32> = fs::read_to_string("input.txt")?
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    data.sort_unstable();

    let part_one = data.last().expect("Must have one element");
    let part_two = data[data.len() - 3..].iter().sum::<u32>();

    println!("Part one: {part_one}");
    println!("Path two: {part_two}");

    Ok(())
}
