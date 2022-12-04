use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use itertools::izip;
use std::str::Lines;
use itertools::Itertools;
use std::collections::HashSet;

fn get_priority (c:char) -> u64{
    match c {
        'a'..='z' => (c as u64) - 96,
        'A'..='Z' => (c as u64)-38,
        _ => 0,
    }
}



fn main() {
    println!("Day 3 Solution");

    let mut file = File::open("input.txt")
         .expect("file not found");
        let mut  data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading from a file");

    let mut result1: u64 = 0;
    let mut result2: u64 = 0;
    'outer: for line in data.lines(){
        let (mut compart1,mut compart2) = line.split_at(line.len()/2);
        'inner1: for c in compart1.chars() {
            'inner2: for c2 in compart2.chars() {
                if c == c2 {
                    if c as u64 == 0{
                        break 'outer;
                    }
                    if c.is_uppercase() {
                        result1 += get_priority(c);
                        break 'inner1;
                    } else {
                        result1 += get_priority(c);
                        break 'inner1;
                    }
                }
            }

        }
    }
    println!("task 1 {}",result1);

    let mut chunks = data.lines().chunks(3).collect();

    println!("{:?}",chunks);

    // for chunk in chunks.into_iter() {
    //     let maps = chunk.map(|line| line.chars().collect()).collect::<char>();

    //     println!("{:?}",maps);
    // }

    // for chunk in chunks.into_iter() {
    //     let maps = chunk
    //         .map(|line| line.chars().collect::<HashSet<char>>())
    //         .collect::<Vec<HashSet<char>>>();

    //     let shared = maps[0]
    //         .iter()
    //         .filter(|b| maps[1..].iter().all(|set| set.contains(b)))
    //         .nth(0)
    //         .unwrap();

    //     result2 += match shared {
    //         'a'..='z' => *shared as u64 - 96,
    //         'A'..='Z' => *shared as u64 - 38,
    //         _ => 0,
    //     }
    // }




    println!("task 2 {}",result2);
}

