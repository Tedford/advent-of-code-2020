use std::cmp;
use std::fs;

#[derive(PartialEq, Eq, Clone, Debug)]
enum State {
    Floor,
    Empty,
    Occupied,
}

fn calibrate_part1() {
    println!("Calibration -- Part 1");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day11.example.txt",
        37,
    )];

    for input in inputs.iter() {
        let value = part1(input.0);
        println!(
            "{0} => {1} [{2}]",
            input.0,
            value,
            match value == input.1 {
                true => "SUCCESS",
                _ => panic!("FAILED.  Expected {} but calculated {}", input.1, value),
            }
        );
    }
}

fn load(path: &str) -> Vec<Vec<State>> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("could not load file {}", path))
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => State::Empty,
                    '.' => State::Floor,
                    '#' => State::Occupied,
                    _ => panic!("Unexpected state {} detected", c),
                })
                .collect()
        })
        .collect()
}

fn count_adjecent_occupancy(area: &[Vec<State>], x: i32, y: i32) -> i32 {
    let mut occupied = 0;

    // print!("\t({},{}) => ", x, y);
    for x2 in cmp::max(0, x - 1)..cmp::min(area[0].len() as i32, x + 2) {
        for y2 in cmp::max(0, y - 1)..cmp::min(area.len() as i32, y + 2) {
            
            occupied += if ((x, y) != (x2, y2)) && area[y2 as usize][x2 as usize] == State::Occupied
            {
                // print!("({},{}) ", x2, y2);
                1
            } else {
                0
            };
        }
    }

    // println!("{}", occupied);

    occupied
}

fn reseat(area: &[Vec<State>]) -> (Vec<Vec<State>>, i32) {
    let mut changes = 0;
    let mut area2: Vec<Vec<State>> = Vec::<Vec<State>>::new();

    for y in 0..area.len() {
        area2.push(vec![]);
        for x in 0..area[y].len() {
            area2[y].push(
                // print!("({},{}): {:?} => ", x, y, &area[y][x]);
                if area[y][x] == State::Floor {
                    State::Floor
                } else {
                    match count_adjecent_occupancy(&area, x as i32, y as i32) {
                        0 => State::Occupied,
                        4..=8 => State::Empty,
                        _ => area[y][x].clone(),
                    }
                },
            );

            changes += if area2[y][x] != area[y][x] { 1 } else { 0 };
            // println!("{:?}", area2[y][x]);
        }
    }

    (area2, changes)
}

fn print_area(area: &[Vec<State>]) {
    for row in area {
        for col in row {
            print!(
                "{}",
                match col {
                    State::Floor => ".",
                    State::Occupied => "#",
                    State::Empty => "L",
                }
            );
        }
        println!();
    }
}

fn part1(path: &str) -> i32 {
    let mut area = load(path);

    let mut done = false;
    let mut iteration = 0;

    while !done {
        let (area2, count) = reseat(&area);
        println!("Iteration {} - {} changes", iteration, count);
        //print_area(&area2);

        done = count == 0;
        area = area2;
        iteration += 1;
    }

    // println!("{:#?}", area);

    let mut occupied = 0;
    for y in 0..area.len() {
        for x in 0..area[y].len() {
            occupied += if area[y][x] == State::Occupied { 1 } else { 0 };
        }
    }
    occupied
}

fn main() {
    println!("Day 11");
    println!("=============");
    // calibrate_part1();

    println!("Part 1 - {} seats occupied", part1(r"c:\projects\github\advent-of-code-2020\data\day11.txt"));

}
