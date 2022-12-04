#![warn(clippy::pedantic)]

use std::ops::RangeInclusive;
use std::str::FromStr;
use std::{error::Error, fs};

trait Overlaps<T> {
    fn overlaps(&self, other: Self) -> bool
    where
        T: PartialOrd;
}

impl<T> Overlaps<T> for RangeInclusive<T> {
    fn overlaps(&self, other: Self) -> bool
    where
        T: PartialOrd,
    {
        self.start() <= other.end() && other.start() <= self.end()
    }
}

trait ToRangeInclusive {
    fn to_range_inclusive<'tmp, T>(
        &self,
        delimiter: char,
    ) -> Result<RangeInclusive<T>, Box<dyn Error + 'tmp>>
    where
        T: FromStr + PartialOrd + Copy,
        <T as FromStr>::Err: Error + 'tmp;
}

impl ToRangeInclusive for &str {
    fn to_range_inclusive<'tmp, T>(
        &self,
        delimiter: char,
    ) -> Result<RangeInclusive<T>, Box<dyn Error + 'tmp>>
    where
        T: FromStr + PartialOrd + Copy,
        <T as FromStr>::Err: Error + 'tmp,
    {
        let (start, end) = match self.split_once(delimiter) {
            Some(x) => Ok(x),
            None => Err("Can't split the string"),
        }?;

        let start = start.parse::<T>()?;
        let end = end.parse::<T>()?;

        Ok(start..=end)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part_one: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let (first, second) = match line.split_once(',') {
                Some(x) => Ok(x),
                None => Err("Can't split the string"),
            }?;

            let first = first.to_range_inclusive::<u32>('-')?;
            let second = second.to_range_inclusive::<u32>('-')?;

            Ok(u32::from(
                first.contains(second.start()) && first.contains(second.end())
                    || second.contains(first.start()) && second.contains(first.end()),
            ))
        })
        .sum::<Result<u32, _>>()?;

    println!("Part one: {part_one}");

    let part_two: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let (first, second) = match line.split_once(',') {
                Some(x) => Ok(x),
                None => Err("Can't split the string"),
            }?;

            let first = first.to_range_inclusive::<u32>('-')?;
            let second = second.to_range_inclusive::<u32>('-')?;

            Ok(u32::from(first.overlaps(second)))
        })
        .sum::<Result<u32, _>>()?;

    println!("Part two: {part_two}");

    Ok(())
}
