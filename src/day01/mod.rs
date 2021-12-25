use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

pub fn part1() -> u32 {
    let file = File::open("./src/day01/input.txt")
        .expect("file was not found");
    let reader = BufReader::new(file);
    let vals: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();
    let mut increased = 0;
    for (i, val) in vals.iter().enumerate() {
        if (i > 0) && (val > &vals[i-1]) {
            increased += 1;
        }
    }
    return increased;
}

pub fn part2() -> u32 {
    let file = File::open("./src/day01/input.txt")
        .expect("file was not found");
    let reader = BufReader::new(file);
    let vals: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();
    let mut increased = 0;
    for i in 3..vals.len() {
        let current_window: u32 = vals[i-2..i+1].iter().sum();
        let previous_window: u32 = vals[i-3..i].iter().sum();
        if current_window > previous_window {
            increased += 1;
        }
    }
    return increased;
}
