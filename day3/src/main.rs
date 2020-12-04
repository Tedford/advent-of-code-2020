use std::fmt;
use std::fs;

#[derive(Copy, Clone)]
struct Movement {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Context {
    filename: String,
    map: Vec<Vec<bool>>,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {0}, y: {1}", self.x, self.y)
    }
}

fn load_map(path: &str) -> Context {
    let map = fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| if c == '.' { false } else { true })
                .collect()
        })
        .collect();

    Context {
        map,
        filename: path
            .split("\\")
            .collect::<Vec<&str>>()
            .split_last()
            .unwrap()
            .0
            .to_string(),
    }
}

fn descend(map: &Vec<Vec<bool>>, y: usize) -> Option<Vec<bool>> {
    if map.len() - 1 < y {
        None
    } else {
        Some(map[y].clone())
    }
}

fn map_descent(map: &Vec<Vec<bool>>, movement: &Movement) -> i64 {
    let mut y = 0;
    let mut x = 0;
    let mut encountered = 0;

    while let Some(row) = descend(map, y) {
        encountered = encountered + if row[x] { 1 } else { 0 };
        y = y + movement.y;
        x = (x + movement.x) % row.len();
    }

    encountered
}

fn main() {
    println!("Day 3");
    println!("=============");

    let contexts = [
        load_map("c:\\projects\\github\\advent-of-code-2020\\data\\day3.example.txt"),
        load_map("c:\\projects\\github\\advent-of-code-2020\\data\\day3.txt"),
    ];

    println!("Part 1");
    for context in contexts.iter() {
        println!(
            "\t{}\t{}",
            map_descent(&context.map, &Movement { x: 3, y: 1 },),
            context.filename
        );
    }

    println!("Part 2");
    let routes = [
        Movement { x: 1, y: 1 },
        Movement { x: 3, y: 1 },
        Movement { x: 5, y: 1 },
        Movement { x: 7, y: 1 },
        Movement { x: 1, y: 2 },
    ];

    let costs = contexts.iter().map(|c| {
        (
            routes
                .iter()
                .map(|r| map_descent(&c.map, r))
                .fold(1, |acc, encountered| acc * encountered),
            c.filename.to_owned(),
        )
    });

    for cost in costs {
        println!("\t{}\t{}", cost.0, cost.1);
    }
}
