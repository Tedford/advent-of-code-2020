use std::fmt;
use std::fs;

struct Movement {
    x: usize,
    y: usize,
}

fn convert(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| if c == '.' { false } else { true })
        .collect()
}

fn load_map(path: &str) -> Vec<Vec<bool>> {
    fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|x| convert(x))
        .collect()
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {0}, y: {1}", self.x, self.y)
    }
}

fn get_filename(path: &str) -> &str {
    let parts: Vec<&str> = path.split("\\").collect();
    parts[parts.len() - 1]
}

fn map_descent(path: &str, movement: Movement) {
    println!("Data File {0}, Movement {1}", get_filename(path), movement);

    let encountered = load_map(path).iter().fold((0, 0), |acc, row| 
        // println!("x = {0}, Tree = {1}",acc.0, row[acc.0]);
        (
            (acc.0 + movement.x) % row.iter().len(),
            acc.1 + (if row[acc.0] { 1 } else { 0 }),
        )
    );

    println!("Encountered {} trees", encountered.1);
}

fn main() {
    println!("Day 3");
    println!("=============");

    map_descent(
        "c:\\projects\\github\\advent-of-code-2020\\data\\day3.example.txt",
        Movement { x: 3, y: 1 },
    );
    map_descent(
        "c:\\projects\\github\\advent-of-code-2020\\data\\day3.txt",
        Movement { x: 3, y: 1 },
    );
}
