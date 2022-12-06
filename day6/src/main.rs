#![warn(clippy::pedantic)]

use std::{
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut vec: Vec<char> = Vec::new();

    let part_one: Vec<usize> = fs::read_to_string("input.txt")?
        .char_indices()
        .filter_map(|(index, char)| {
            if vec.len() == 4 {
                return Some(index);
            }

            if let Some(i) = vec.iter().position(|&x| x == char) {
                vec.drain(0..=i);
            }
            vec.push(char);
            None
        })
        .collect();

    vec.clear();
    println!("{:#?}", part_one.first());

    let part_two: Vec<usize> = fs::read_to_string("input.txt")?
        .char_indices()
        .filter_map(|(index, char)| {
            if vec.len() == 14 {
                return Some(index);
            }

            if let Some(i) = vec.iter().position(|&x| x == char) {
                vec.drain(0..=i);
            }
            vec.push(char);
            None
        })
        .collect();

    vec.clear();
    println!("{:#?}", part_two.first());

    Ok(())
}
