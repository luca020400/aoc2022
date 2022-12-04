#![warn(clippy::pedantic)]

use std::ops::RangeInclusive;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let part_one: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let split_into_range = |str: &str| -> RangeInclusive<_> {
                let (left, right) = str.split_once('-').unwrap();
                left.parse::<u32>().unwrap()..=right.parse::<u32>().unwrap()
            };
            let (first, second) = (split_into_range(first), split_into_range(second));
            u32::from(
                first.contains(second.start()) && first.contains(second.end())
                    || second.contains(first.start()) && second.contains(first.end()),
            )
        })
        .sum();

    println!("Part one: {part_one}");

    let part_two: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let split_into_range = |str: &str| -> RangeInclusive<_> {
                let (left, right) = str.split_once('-').unwrap();
                left.parse::<u32>().unwrap()..=right.parse::<u32>().unwrap()
            };
            let (first, second) = (split_into_range(first), split_into_range(second));
            u32::from(
                first
                    .into_iter()
                    .filter(|value| second.contains(value))
                    .count()
                    > 0,
            )
        })
        .sum();

    println!("Part two: {part_two}");

    Ok(())
}
