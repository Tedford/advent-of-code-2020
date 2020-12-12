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

fn calibrate_part2() {
    println!("Calibration -- Part 2");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day11.example.txt",
        26,
    )];

    for input in inputs.iter() {
        let value = part2(input.0);
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

    occupied
}

fn scan_for_occupancy<T>(
    area: &[Vec<State>],
    x: i32,
    y: i32,
    max_x: i32,
    max_y: i32,
    step: T,
) -> i32
where
    T: Fn(&mut i32, &mut i32),
{
    // print!("({},{}) ", x, y);
    let mut occupied = 0;
    let mut x2 = x;
    let mut y2 = y;
    let mut found = false;
    while !found {
        step(&mut x2, &mut y2);
        // print!("({},{}) ", x2, y2);

        if y2 < 0 || x2 < 0 || y2 > max_y - 1 || x2 > max_x - 1 {
            found = true;
        } else {
            found = match area[y2 as usize][x2 as usize] {
                State::Occupied => {
                    occupied += 1;
                    true
                }
                State::Empty => true,
                _ => false,
            }
        }
    }
    // println!("{}", occupied);
    occupied
}

fn count_visible_occupancy(area: &[Vec<State>], x: i32, y: i32) -> i32 {
    let max_y = area.len() as i32;
    let max_x = area[0].len() as i32;

    // NW
    scan_for_occupancy(area, x, y, max_x, max_y, |_x2, _y2| {
        *_x2 -= 1;
        *_y2 -= 1;
    }) +
    // N
    scan_for_occupancy(area, x, y, max_x, max_y, |_x2, _y2| {
        *_y2 -= 1;
    }) +
    // NE
    scan_for_occupancy(area, x, y, max_x, max_y, |_x2, _y2| {
        *_x2 += 1;
        *_y2 -= 1;
    }) +
    // E
    scan_for_occupancy(area, x, y, max_x, max_y, |_x2, _y2| {
        *_x2 += 1;
    }) +
    // SE
    scan_for_occupancy(area, x, y, max_x, max_y, |_x2, _y2| {
        *_x2 += 1;
        *_y2 += 1;
    })
    // S
    + scan_for_occupancy(area, x, y, max_x, max_y, |_x2, _y2| {
        *_y2 += 1;
    }) +
    // SW
    scan_for_occupancy(area, x, y, max_x, max_y, |_x2, _y2| {
        *_x2 -= 1;
        *_y2 += 1;
    })
    // W
    + scan_for_occupancy(area, x, y, max_x, max_y, |_x2, _y2| {
        *_x2 -= 1;
    })
}

fn reseat<T>(area: &[Vec<State>], min_occupancy: i32, count_occupancy: T) -> (Vec<Vec<State>>, i32)
where
    T: Fn(&[Vec<State>], i32, i32) -> i32,
{
    let mut changes = 0;
    let mut area2: Vec<Vec<State>> = Vec::<Vec<State>>::new();

    for y in 0..area.len() {
        area2.push(vec![]);
        for x in 0..area[y].len() {
            area2[y].push(if area[y][x] == State::Floor {
                State::Floor
            } else {
                let count = count_occupancy(&area, x as i32, y as i32);
                if count == 0 {
                    State::Occupied
                } else if count > min_occupancy {
                    State::Empty
                } else {
                    area[y][x].clone()
                }
            });

            changes += if area2[y][x] != area[y][x] { 1 } else { 0 };
        }
    }

    (area2, changes)
}

#[allow(dead_code)]
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
        let (area2, count) = reseat(&area, 3, count_adjecent_occupancy);
        println!("Iteration {} - {} changes", iteration, count);
        done = count == 0;
        area = area2;
        iteration += 1;
    }

    let mut occupied = 0;
    for row in area {
        for seat in row {
            occupied += if seat == State::Occupied { 1 } else { 0 };
        }
    }
    occupied
}

fn part2(path: &str) -> i32 {
    let mut area = load(path);

    let mut done = false;
    let mut iteration = 0;

    while !done {
        let (area2, count) = reseat(&area, 4, count_visible_occupancy);
        println!("Iteration {} - {} changes", iteration, count);
        // print_area(&area2);

        done = count == 0;
        area = area2;
        iteration += 1;
    }

    let mut occupied = 0;
    for row in area {
        for seat in row {
            occupied += if seat == State::Occupied { 1 } else { 0 };
        }
    }

    occupied
}

fn main() {
    println!("Day 11");
    println!("=============");
    calibrate_part1();

    println!(
        "Part 1 - {} seats occupied",
        part1(r"c:\projects\github\advent-of-code-2020\data\day11.txt")
    );

    calibrate_part2();

    println!(
        "Part 2 - {} seats occupied",
        part2(r"c:\projects\github\advent-of-code-2020\data\day11.txt")
    );
}
