#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let mut path: VecDeque<String> = VecDeque::new();

    fs::read_to_string("input.txt")?
        .split('\n')
        .for_each(|line| {
            let split: Vec<&str> = line.split(' ').collect();
            match split.len() {
                3 => match split[2] {
                    ".." => {
                        path.pop_back();
                    }
                    relative_path => {
                        path.push_back(relative_path.to_string());
                    }
                },
                2 => match split[0].parse::<u32>() {
                    Ok(size) => {
                        let work = path.make_contiguous();
                        for i in 0..work.len() {
                            let key = &work[..work.len() - i].join("/");
                            map.entry(key.clone()).or_insert(0);
                            map.insert(key.clone(), size + map[key]);
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        });

    let part_one: u32 = map.values().filter(|x| **x <= 100000).sum();

    println!("{:#?}", part_one);

    let root_size = map["/"];
    let part_two: Option<&u32> = map
        .values()
        .filter(|x| root_size - **x <= 70000000 - 30000000)
        .min();

    println!("{:#?}", part_two);

    Ok(())
}
