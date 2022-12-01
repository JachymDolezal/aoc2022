use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("------- AoC 2022 day 1 solution -------");


    let mut calorysum: u64 = 0;
    let mut mostcalories: u64 = 0;
    let mut mostcalories2: u64 = 0;
    let mut mostcalories3: u64 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        'outer: for line in lines {
            if let Ok(num) = line {
                if !num.eq(""){
                    let calory: u64 = num.parse().unwrap();
                    if calory == 10000{
                        println!("hit");
                    }
                    calorysum = calorysum + calory;
                } else{
                    //solution for task 1
                    if calorysum > mostcalories{
                        mostcalories3 = mostcalories2;
                        mostcalories2 = mostcalories;
                        mostcalories = calorysum;

                    } else if calorysum > mostcalories2{
                        mostcalories3 = mostcalories2;
                        mostcalories2 = calorysum;
                    } else if calorysum > mostcalories3 {
                        mostcalories3 = calorysum;
                    }

                    calorysum = 0;
                }
            }

        }

    }
    if calorysum > mostcalories{
        mostcalories3 = mostcalories2;
        mostcalories2 = mostcalories;
        mostcalories = calorysum;

    } else if calorysum > mostcalories2{
        mostcalories3 = mostcalories2;
        mostcalories2 = calorysum;
    } else if calorysum > mostcalories3 {
        mostcalories3 = calorysum;
    }
    println!("TASK1: Most Calories: {}",mostcalories);
    let sum: u64 = mostcalories + mostcalories2 + mostcalories3;
    println!("TASK2: Top3 Calories: {}",sum);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}