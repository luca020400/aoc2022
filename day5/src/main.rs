#![warn(clippy::pedantic)]

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, error::Error, fs};

fn str_strip_numbers(s: &str) -> Vec<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.find_iter(s)
        .filter_map(|digits| digits.as_str().parse::<usize>().ok())
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stacks_one = vec![VecDeque::new(); 9];

    match fs::read_to_string("input.txt")?
        .split("\n\n")
        .tuples::<(_, _)>()
        .exactly_one()
    {
        Ok((creates, moves)) => {
            creates.split('\n').for_each(|line| {
                for x in (1..line.len()).step_by(4) {
                    let char = line.chars().nth(x).unwrap();
                    if char.is_ascii_uppercase() {
                        stacks_one[x / 4].push_back(char);
                    }
                }
            });

            moves
                .split('\n')
                .filter(|&x| !x.is_empty())
                .for_each(|line| {
                    let vec = str_strip_numbers(line);
                    let (n, from, to) = vec.iter().collect_tuple().unwrap();
                    for _ in 0..*n {
                        if let Some(popped) = stacks_one[from - 1].pop_front() {
                            stacks_one[to - 1].push_front(popped);
                        }
                    }
                });
        }
        Err(_) => return Err("Not exactly one".into()),
    }

    let part_one = stacks_one
        .iter()
        .map(|queue| queue.front().unwrap())
        .collect::<String>();

    println!("Part one: {part_one}");

    let mut stacks_two = vec![VecDeque::new(); 9];

    match fs::read_to_string("input.txt")?
        .split("\n\n")
        .tuples::<(_, _)>()
        .exactly_one()
    {
        Ok((creates, moves)) => {
            creates.split('\n').for_each(|line| {
                for x in (1..line.len()).step_by(4) {
                    let char = line.chars().nth(x).unwrap();
                    if char.is_ascii_uppercase() {
                        stacks_two[x / 4].push_back(char);
                    }
                }
            });

            moves
                .split('\n')
                .filter(|&x| !x.is_empty())
                .for_each(|line| {
                    let vec = str_strip_numbers(line);
                    let (n, from, to) = vec.iter().collect_tuple().unwrap();

                    let mut work = VecDeque::<char>::new();
                    for _ in 0..*n {
                        if let Some(popped) = stacks_two[from - 1].pop_front() {
                            work.push_front(popped);
                        }
                    }
                    for elem in work.drain(..) {
                        stacks_two[to - 1].push_front(elem);
                    }
                });
        }
        Err(_) => return Err("Not exactly one".into()),
    }

    let part_two = stacks_two
        .iter()
        .map(|queue| queue.front().unwrap())
        .collect::<String>();

    println!("Part two: {part_two}");

    Ok(())
}
