use std::fs;

fn load(path: &str) -> (i32, Vec<i32>) {
    let lines: Vec<String> = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("unable to load the file {}", path))
        .lines()
        .map(|l| l.to_string())
        .collect();

    let time = lines[0]
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Unable to parse the arrival time"));

    let buses = lines[1]
        .split(',')
        .filter(|c| *c != "x")
        .map(|c| {
            c.parse::<i32>()
                .unwrap_or_else(|_| panic!("Unable to parse bus time {}", c))
        })
        .collect();

    (time, buses)
}

fn main() {
    println!("Day 13");
    println!("=============");
    println!("{:?}",load(r"c:\projects\github\advent-of-code-2020\data\day13.txt"));
}
