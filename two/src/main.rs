fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    let lines: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    for line in lines {
        let instruction = line[0];
        let value: i32 = line[1].parse().unwrap();

        match instruction {
            "forward" => {
                horizontal += value;
            },
            "down" => {
                depth += value;
            },
            "up" => {
                depth -= value;
            },
            _ => ()
        }
    }

    println!("{}", horizontal * depth);
}

pub fn part_two() {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    let lines: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    for line in lines {
        let instruction = line[0];
        let value: i32 = line[1].parse().unwrap();

        match instruction {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            },
            "down" => {
                aim += value;
            },
            "up" => {
                aim -= value;
            },
            _ => ()
        }
    }

    println!("{}", horizontal * depth);
}
