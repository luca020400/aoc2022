#![warn(clippy::pedantic)]

use std::collections::HashSet;
use std::{collections::HashMap, error::Error, fs};

use std::ops::{Add, AddAssign, Sub};
use std::{cmp, vec};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

const CENTER: Point = Point { x: 0, y: 0 };

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

trait Clamp<T> {
    fn clamp(input: Self, min: Self, max: Self) -> Self
    where
        T: cmp::Ord;
}

impl<T> Clamp<T> for T {
    fn clamp(input: Self, min: Self, max: Self) -> Self
    where
        T: cmp::Ord,
    {
        cmp::min(min, cmp::max(max, input))
    }
}

fn follow(h: Point, t: Point) -> Point {
    let distance = h - t;

    let difference = Point {
        x: distance.x.clamp(-1, 1),
        y: distance.y.clamp(-1, 1),
    };

    if distance == difference {
        t
    } else {
        t + difference
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let moves = HashMap::from([
        ("R", Point { x: 1, y: 0 }),
        ("L", Point { x: -1, y: 0 }),
        ("U", Point { x: 0, y: 1 }),
        ("D", Point { x: 0, y: -1 }),
    ]);

    let mut positions: HashSet<Point> = HashSet::new();

    let mut head = CENTER;
    let mut tail = CENTER;

    fs::read_to_string("input.txt")?.lines().for_each(|line| {
        let (dir, amount) = line.split_once(' ').unwrap();
        let step = amount.parse::<u32>().unwrap();
        for _ in 0..step {
            head += moves[dir];
            tail = follow(head, tail);
            positions.insert(tail);
        }
    });

    println!("Part one {:#?}", positions.len());

    positions.clear();
    let mut ropes = vec![CENTER; 10];

    fs::read_to_string("input.txt")?.lines().for_each(|line| {
        let (dir, amount) = line.split_once(' ').unwrap();
        let step = amount.parse::<u32>().unwrap();

        for _ in 0..step {
            // This is the head
            ropes[0] += moves[dir];

            // For each other rope make sure it's following its
            // corrispective head properly
            for i in 0..ropes.len() - 1 {
                ropes[i + 1] = follow(ropes[i], ropes[i + 1]);
            }

            // The last rope is what we care about
            positions.insert(ropes.last().unwrap().clone());
        }
    });

    println!("Part two {:#?}", positions.len());

    Ok(())
}
