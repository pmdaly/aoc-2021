use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

pub fn part1() -> u32 {
    let file = File::open("./src/day01/input.txt").expect("file was not found");
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
