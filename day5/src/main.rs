use std::fmt;
use std::fs;
use substring::Substring;

struct Assignment {
    row: i32,
    column: i32,
}

impl Assignment {
    fn id(&self) -> i32 {
        self.row * 8 + self.column
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "row {0}, column {1}, seat ID {2}",
            self.row,
            self.column,
            self.id()
        )
    }
}

fn bifrucate(min: i32, max: i32) -> i32 {
    ((max as f32 - min as f32) / 2.0).ceil() as i32 + min
}

fn bottom(range: (i32, i32)) -> (i32, i32) {
    (range.0, bifrucate(range.0, range.1))
}

fn top(range: (i32, i32)) -> (i32, i32) {
    (bifrucate(range.0, range.1), range.1)
}

fn bifucate_range(code: String, range: (i32, i32)) -> (i32, i32) {
    code.chars().into_iter().fold(range, |acc, c| match c {
        'F' => bottom(acc),
        'L' => bottom(acc),
        'B' => top(acc),
        'R' => top(acc),
        _ => panic!("Invalid letter encountered"),
    })
}

fn calibrate_part1() {
    println!("Calibration");
    let inputs = [
        ("FBFBBFFRLR".to_owned(), 357),
        ("BFFFBBFRRR".to_owned(), 567),
        ("FFFBBBFRRR".to_owned(), 119),
        ("BBFFBBFRLL".to_owned(), 820),
    ];

    for input in inputs.iter() {
        let assignment = calculate_assignment(input.0.clone());
        println!(
            "{0} => {1} [{2}]",
            input.0,
            assignment,
            match assignment.id() == input.1 {
                true => "SUCCESS",
                _ => "FAILED",
            }
        );
    }
}

fn calculate_assignment(code: String) -> Assignment {
    let row = bifucate_range(code.substring(0, 7).to_owned(), (1, 128));
    let column = bifucate_range(code.substring(7, 10).to_owned(), (1, 8));

    Assignment {
        row: row.0 - 1,
        column: column.0 - 1,
    }
}

fn load_assignments(path: &str) -> Vec<Assignment> {
    fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|l| calculate_assignment(l.to_owned()))
        .collect::<Vec<Assignment>>()
}

fn main() {
    println!("Day 5");
    println!("=============");
    calibrate_part1();
    let mut assignments = load_assignments(r"C:\Projects\GitHub\advent-of-code-2020\Data\day5.txt");
    assignments.sort_by(|a, b| b.id().cmp(&a.id()));
    println!("Max seat {}", assignments[0]);

    // find the hole
    for i in 0..assignments.len() - 1 {
        if assignments[i+1].id() != assignments[i].id()-1 {
            println!("Discontinuity found between {} and {}",assignments[i], assignments[i+1]);
        }
    }
}
