use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Rule {
    color: String,
    contents: HashMap<String, i32>,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0}: {{ {1} }}",
            self.color,
            self.contents
                .iter()
                .map(|(k, v)| format!("{}:{}", k, v))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

fn parse_contents(s: &str) -> HashMap<String, i32> {
    if s.starts_with("no") {
        HashMap::new()
    } else {
        s.split(", ")
            .map(|c| {
                let mut parts = c.trim().split(' ');
                let count = parts.next().unwrap().parse::<i32>().unwrap();
                let name = format!("{} {}", parts.next().unwrap(), parts.next().unwrap());
                (name, count)
            })
            .collect::<HashMap<String, i32>>()
    }
}

fn decompose(s: &str) -> Rule {
    let mut parts = s.split("bags contain");
    let color = parts.next().unwrap().trim().to_string();

    Rule {
        color: color,
        contents: parse_contents(parts.next().unwrap().trim()),
    }
}

fn load(path: &str) -> Vec<Rule> {
    fs::read_to_string(path)
        .expect(path)
        .lines()
        .map(|l| decompose(l))
        .collect()
}

fn find_parent(rules: &Vec<Rule>, color: &str) -> Vec<String> {
    rules
        .iter()
        .filter(|r| r.contents.contains_key(color))
        .map(|r| r.color.clone())
        .collect()
}

fn can_contain(rules: &Vec<Rule>, color: &str) -> i32 {
    let mut inspectable = Some(find_parent(&rules, color));

    let mut candidates: HashSet<String> = HashSet::new();

    while let Some(parents) = inspectable {
        let mut next = Vec::<String>::new();

        for parent in parents {
            candidates.insert(parent.clone());
            for grandparent in find_parent(&rules, &parent) {
                next.push(grandparent);
            }
        }

        match next.len() {
            0 => inspectable = None,
            _ => inspectable = Some(next),
        }
    }

    candidates.len() as i32
}

fn calibrate_part1() {
    println!("Calibration");
    let inputs = [
        (
            r"c:\projects\github\advent-of-code-2020\data\day7.example.txt",
            4,
        ),
    ];

    for input in inputs.iter() {
        let rules = load(input.0);
        let count = can_contain(&rules, "shiny gold");
        println!(
            "{0} => {1} [{2}]",
            input.0,
            count,
            match count == input.1 {
                true => "SUCCESS",
                _ => "FAILED",
            }
        );
    }
}

fn main() {
    println!("Day 7");
    println!("=============");

    calibrate_part1();

    let rules = load(r"c:\projects\github\advent-of-code-2020\data\day7.txt");
    println!("Part 1: {}", can_contain(&rules,"shiny gold"));

    // let mut inspectable = Some(find_parent(&rules, "shiny gold"));

    // let mut candidates: HashSet<String> = HashSet::new();

    // while let Some(parents) = inspectable {
    //     let mut next = Vec::<String>::new();

    //     for parent in parents {
    //         candidates.insert(parent.clone());
    //         for grandparent in find_parent(&rules, &parent) {
    //             next.push(grandparent);
    //         }
    //     }

    //     match next.len() {
    //         0 => inspectable = None,
    //         _ => inspectable = Some(next),
    //     }
    // }

    // println!("{} can hold {}", candidates.len(), "shiny gold");
}
