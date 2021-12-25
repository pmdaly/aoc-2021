use std::fs;


enum Direction {
    Forward,
    Up,
    Down,
}

struct Movement {
    direction: Direction,
    distance: i32
}

pub fn part1() -> i32 {
    let data = fs::read_to_string("./src/day02/input.txt")
        .expect("file was not found");
    let movements: Vec<Movement> = data
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(' ').unwrap();
            Movement { 
                direction: match direction {
                    "up" => Direction::Up,
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    _ => panic!("direction must be of Forward, Down or Up")
                },
                distance: distance.parse::<i32>().unwrap() 
            }
        }).collect();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for movement in movements {
        match movement.direction {
            Direction::Up => y -= movement.distance,
            Direction::Down => y += movement.distance,
            Direction::Forward => x += movement.distance,
        }
    }
    return x*y;
}


