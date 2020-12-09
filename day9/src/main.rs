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
            println!(
                "Checking [{2}..{3}] for [{0}] {1}",
                current, self.bytes[current], start, end
            );

            let window: Vec<i64> = self.bytes[start..end].into_iter().copied().collect();

            valid = match XmasCracker::has_solution(window, self.bytes[current]) {
                true => {
                    current = current + 1;
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

    fn has_solution(window: Vec<i64>, target: i64) -> bool {
        for i in 0..window.len() {
            for j in i + 1..window.len() {
                if window[i] + window[j] == target {
                    println!("\t{0} + {1} == {2}", window[i], window[j], target);
                    return true;
                }
            }
        }
        return false;
    }
}

fn load(path: &str) -> Vec<i64> {
    fs::read_to_string(path)
        .expect(format!("Could not load {}", path).as_str())
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

fn main() {
    println!("Day 8");
    println!("=============");

    calibrate_part1();

    let cracker = XmasCracker::new(
        25,
        25,
        load(r"c:\projects\github\advent-of-code-2020\data\day9.txt"),
    );

    // 661 too low
    match cracker.find_first_invalid() {
        Some(x) => println!("Part 1 - {}", x),
        None => panic!("Unable to find a discontinutiy in part 1"),
    };
}
