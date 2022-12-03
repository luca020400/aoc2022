#![warn(clippy::pedantic)]

use std::fs;

fn main() {
    let mut data: Vec<u32> = fs::read_to_string("input.txt")
        .expect("File not found")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u32>().expect("Can't convert to u32"))
                .sum()
        })
        .collect();

    data.sort_unstable();

    let max = data.last().expect("Must have one element");
    let max3 = data[data.len() - 3..].iter().sum::<u32>();

    println!("Part one: {max}");
    println!("Path two: {max3}");
}
