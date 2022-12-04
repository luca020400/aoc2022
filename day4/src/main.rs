#![warn(clippy::pedantic)]

use std::ops::RangeInclusive;
use std::{error::Error, fs};

trait Overlap<U = Self> {
    fn overlaps(&self, other: U) -> bool;
}

impl<T: std::cmp::PartialOrd> Overlap for RangeInclusive<T> {
    fn overlaps(&self, other: Self) -> bool {
        self.start() <= other.end() && other.start() <= self.end()
    }
}

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
            u32::from(first.overlaps(second))
        })
        .sum();

    println!("Part two: {part_two}");

    Ok(())
}
