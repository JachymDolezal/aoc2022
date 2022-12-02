use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/*
A X rock 1
B Y paper 2
C Z scissors 3

*/

/*

*/
enum Play{
    Rock,
    Paper,
    Scissors,
    None,
}

fn char_to_enum(c: char) -> Play {
    match c {
        'X' => Play::Rock,
        'A' => Play::Rock,
        'Y' => Play::Paper,
        'B' => Play::Paper,
        'Z' => Play::Scissors,
        'C' => Play::Scissors,
        _ => Play::None,
    }
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

fn game_result_1(player1: Play,player2: Play) -> u64 {
    match player1 {
        Play::Rock => {
            match player2 {
                Play::Rock => 4,
                Play::Paper => 8,
                Play::Scissors => 3,
                Play::None => 0,
            }
        },
        Play::Paper => {
            match player2 {
                Play::Rock => 1,
                Play::Paper => 5,
                Play::Scissors => 9,
                Play::None => 0,
            }
        },
        Play::Scissors => {
            match player2 {
                Play::Rock => 7,
                Play::Paper => 2,
                Play::Scissors => 6,
                Play::None => 0,
            }
        },
        Play::None => 0,
    }
}

/*
lose -> ROCK
draw -> Paper
win -> Scissors
*/

fn game_result_2(player1: Play,player2: Play) -> u64 {
    match player1 {
        Play::Rock => {
            match player2 {
                Play::Rock => 3,
                Play::Paper => 4,
                Play::Scissors => 8,
                Play::None => 0,
            }
        },
        Play::Paper => {
            match player2 {
                Play::Rock => 1,
                Play::Paper => 5,
                Play::Scissors => 9,
                Play::None => 0,
            }
        },
        Play::Scissors => {
            match player2 {
                Play::Rock => 2,
                Play::Paper => 6,
                Play::Scissors => 7,
                Play::None => 0,
            }
        },
        Play::None => 0,
    }
}

fn main() {
    println!("Day 2 solution");

    let mut file = File::open("./input.txt")
         .expect("file not found");
        let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading from a file");

    data = data.replace(" ","");
    let mut i = 0;
    let mut result1: u64 = 0;
    let mut result2: u64 = 0;
    for line in data.lines(){
        result1 += game_result_1(char_to_enum(line.chars().nth(0).unwrap()),char_to_enum(line.chars().nth(1).unwrap()));
        result2 += game_result_2(char_to_enum(line.chars().nth(0).unwrap()),char_to_enum(line.chars().nth(1).unwrap()));
        // println!("{}",line);
        // i += 1;
        // println!("hit {}", line);
        // println!("hit {}", i);
    }
    println!("{}",result1);
    println!("{}",result2);
}
