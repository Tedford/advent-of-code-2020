use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

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
            .collect()
    }
}

fn decompose(s: &str) -> (String, HashMap<String, i32>) {
    let mut parts = s.split("bags contain");
    let color = parts.next().unwrap().trim().to_string();

    (color, parse_contents(parts.next().unwrap().trim()))
}

fn load(path: &str) -> HashMap<String, HashMap<String, i32>> {
    fs::read_to_string(path)
        .expect(path)
        .lines()
        .map(|l| decompose(l))
        .collect()
}

fn find_parent(rules: &HashMap<String, HashMap<String, i32>>, color: &str) -> Vec<String> {
    rules
        .iter()
        .filter(|(_, v)| v.contains_key(color))
        .map(|(k, _)| k.clone())
        .collect()
}

// replace with fold
fn can_contain(rules: &HashMap<String, HashMap<String, i32>>, color: &str) -> i32 {
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

fn count_children(rules: &HashMap<String, HashMap<String, i32>>, color: &str) -> i32 {
    let children = rules.get(color).unwrap();
    children
        .iter()
        .map(|(color, count)| count + count * count_children(rules, color))
        .sum::<i32>()
}

fn calibrate_part1() {
    println!("Calibration");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day7.example.1.txt",
        4,
    )];

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

fn calibrate_part2() {
    println!("Calibration");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day7.example.2.txt",
        126,
    )];

    for input in inputs.iter() {
        let rules = load(input.0);
        let count = count_children(&rules, "shiny gold");
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
    let rules = load(r"c:\projects\github\advent-of-code-2020\data\day7.txt");

    calibrate_part1();
    println!("Part 1: {}", can_contain(&rules, "shiny gold"));
    calibrate_part2();
    println!("Part 2: {}", count_children(&rules, "shiny gold"));
}
