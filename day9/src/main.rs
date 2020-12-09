use std::fs;

struct XmasCracker {
    preamble: i32,
    window: i32,
    bytes: Vec<i64>,
}

impl XmasCracker {
    pub fn new(preamble: i32, window: i32, bytes: Vec<i64>) -> Self {
        XmasCracker {
            preamble,
            window,
            bytes,
        }
    }

    pub fn find_first_invalid(&self) -> Option<i64> {
        let mut current = (self.preamble) as usize;
        let mut valid = true;

        while valid && current < self.bytes.len() {
            let start = current - self.window as usize;
            let end = current;
            let window: Vec<i64> = self.bytes[start..end].iter().copied().collect();

            valid = match XmasCracker::has_solution(window, self.bytes[current]) {
                true => {
                    current += 1;
                    true
                }
                false => false,
            }
        }

        match valid {
            true => None,
            false => Some(self.bytes[current]),
        }
    }

    pub fn find_contiguous_range(&self, target: i64) -> Option<(Vec<i64>, i64)> {
        let mut done = false;
        let mut range = None;
        let mut window = 2;

        while !done {
            println!("\tChecking window size {} to find {}", window, target);
            let candidates: Vec<Vec<i64>> = self
                .bytes
                .windows(window)
                .map(|w| (w.iter().sum::<i64>(), w.iter().copied().collect::<Vec<_>>()))
                .filter(|w| w.0 == target)
                .map(|w| w.1)
                .collect();

            done = match candidates.len() {
                0 => {
                    if window < 50 {
                        window += 1;
                        false
                    } else {
                        true
                    }
                }
                1 => {
                    let mut sorted = candidates[0].clone();
                    sorted.sort_unstable();
                    let crc = sorted[0] + sorted[sorted.len() - 1];
                    range = Some((candidates[0].clone(), crc));
                    true
                }
                _ => panic!("{} solutions found for target {}", candidates.len(), target),
            }
        }

        range
    }

    fn has_solution(window: Vec<i64>, target: i64) -> bool {
        for i in 0..window.len() {
            for j in i + 1..window.len() {
                if window[i] + window[j] == target {
                    return true;
                }
            }
        }
        false
    }
}

fn load(path: &str) -> Vec<i64> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not load {}", path))
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect()
}

fn calibrate_part1() {
    println!("Calibration -- Part 1");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day9.example.txt",
        5,
        5,
        127,
    )];

    for input in inputs.iter() {
        let cracker = XmasCracker::new(input.1, input.2, load(input.0));

        let value = cracker.find_first_invalid().unwrap();
        println!(
            "{0} => {1} [{2}]",
            input.0,
            value,
            match value == input.3 {
                true => "SUCCESS",
                _ => "FAILED",
            }
        );
    }
}

fn calibrate_part2() {
    println!("Calibration -- Part 2");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day9.example.txt",
        25,
        25,
        127,
        (vec![15, 25, 47, 40], 62),
    )];

    for input in inputs.iter() {
        let cracker = XmasCracker::new(input.1, input.2, load(input.0));

        match cracker.find_contiguous_range(input.3) {
            None => panic!("Calibration failed, no solution found."),
            Some(value) => {
                println!(
                    "{0} => {1:?} [{2}]",
                    input.0,
                    value,
                    match value == input.4 {
                        true => "SUCCESS",
                        _ => "FAILED",
                    }
                );
            }
        }
    }
}

fn main() {
    println!("Day 8");
    println!("=============");

    calibrate_part1();

    let cracker = XmasCracker::new(
        25,
        25,
        load(r"c:\projects\github\advent-of-code-2020\data\day9.txt"),
    );

    let target = match cracker.find_first_invalid() {
        Some(x) => {
            println!("The sequence discontinunity is {}", x);
            x
        }
        None => panic!("Unable to find a discontinutiy in part 1"),
    };

    calibrate_part2();

    match cracker.find_contiguous_range(target) {
        Some(x) => println!(
            "The encryption weakness is {0} derived from {1:?}",
            x.1, x.0
        ),
        None => panic!("Unable to find the weakness"),
    }
}
