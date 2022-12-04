use std::fs::File;
use std::io::Read;

fn isfullycontained(first_start:u32,first_end:u32,second_start:u32,second_end:u32) -> bool {

    if first_start <= second_start && first_end >= second_end {
        return true
    }

    return false
}

fn isoverlapping(first_start:u32,first_end:u32,second_start:u32,second_end:u32) -> bool {

    if first_start <= second_end && first_end >= second_start {
        return true
    }


    return false
}

fn main() {
    println!("Day 4 Solution");
    let mut overlapping = 0;
    let mut fullycontained = 0;

    let mut file = File::open("./input.txt")
         .expect("file not found");
        let mut  data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading from a file");

    for line in data.lines(){
        // println!("{}",line);
        let vec = line.split(',')
            .collect::<Vec<&str>>();
        let vec_numbers1 = vec[0].split('-').collect::<Vec<&str>>();
        let vec_numbers2 = vec[1].split('-').collect::<Vec<&str>>();
        let first_start = vec_numbers1[0].parse().unwrap();
        let first_end = vec_numbers1[1].parse().unwrap();
        let second_start = vec_numbers2[0].parse().unwrap();
        let second_end = vec_numbers2[1].parse().unwrap();
        if isfullycontained(first_start,first_end,second_start,second_end) || isfullycontained(second_start,second_end,first_start,first_end) {
            fullycontained += 1;
        } else {
        }
        if isoverlapping(first_start,first_end,second_start,second_end) {
            overlapping += 1;
        } else {
        }
    }
    println!("task1 {} task2 {}",fullycontained, overlapping);
}
