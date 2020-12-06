use std::collections::HashSet;
use std::fs;

fn calibrate_part1() {
    println!("Calibration");
    let inputs = [
        (r"c:\projects\github\advent-of-code-2020\data\day6.example.1.txt", 6),
        (r"c:\projects\github\advent-of-code-2020\data\day6.example.2.txt", 11),
    ];

    for input in inputs.iter() {
        let count= count_affirmatives(get_input(input.0.clone()));
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

fn count_affirmatives(responses: Vec<String>) -> i32 {
    responses.iter().map(|f| f.len() as i32).sum::<i32>()
}

fn condense(raw: &str) -> String {
    let mut map: HashSet<char> = HashSet::with_capacity(26);
 
    for c in raw.chars() {
        if c.is_alphabetic() {
            map.insert(c);
        }
    }

    map.into_iter().collect()
}

fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(path)
        .split("\r\n\r\n")
        .map(|l| condense(l))
        .collect()
}

fn main() {
    println!("Day 6");
    println!("=============");

    calibrate_part1();

    let forms = get_input(r"c:\projects\github\advent-of-code-2020\data\day6.txt");
    println!("Sum of forms {}", count_affirmatives(forms));
}
