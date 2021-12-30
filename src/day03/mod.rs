use std::fs;

pub fn part1() -> u32 {
    let data = fs::read_to_string("./src/day03/input.txt")
        .expect("file was not found");
    let mut bits = vec![0; 12];
    for line in data.lines() {
        for (i, bit) in line.chars().enumerate() {
            bits[i] += match bit {'1' => 1, '0' => -1, _ => panic!("bro...")};
        }
    }
    let gamma = bits
        .iter()
        .map(|x| if *x > 0 { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");
    let gamma_num = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = bits
        .iter()
        .map(|x| if *x <= 0 { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");
    let epsilon_num = u32::from_str_radix(&epsilon, 2).unwrap();
    return gamma_num * epsilon_num;
}

pub fn part2() -> u32 {
    let data = fs::read_to_string("./src/day03/input.txt")
        .expect("file was not found");
    for _line in data.lines() {
        todo!();
    }
    return 0;
}
