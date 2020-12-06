use std::collections::HashMap;
use std::fs;

#[derive(Clone,Debug)]
struct CustomsForm {
    claims: HashMap<char, i32>,
    members: i32,
}

fn calibrate_part1() {
    println!("Calibration");
    let inputs = [
        (
            r"c:\projects\github\advent-of-code-2020\data\day6.example.1.txt",
            6,
        ),
        (
            r"c:\projects\github\advent-of-code-2020\data\day6.example.2.txt",
            11,
        ),
    ];

    for input in inputs.iter() {
        let count = count_individual_affirmatives(get_input(input.0.clone()));
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
    let inputs = [
        (
            r"c:\projects\github\advent-of-code-2020\data\day6.example.1.txt",
            3,
        ),
        (
            r"c:\projects\github\advent-of-code-2020\data\day6.example.2.txt",
            6,
        ),
    ];

    for input in inputs.iter() {
        let count = count_group_affirmatives(get_input(input.0.clone()));
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

fn count_individual_affirmatives(responses: Vec<CustomsForm>) -> i32 {
    responses.iter().map(|f| f.claims.len() as i32).sum::<i32>()
}

fn count_group_affirmatives(responses: Vec<CustomsForm>) -> i32 {
    responses
        .iter()
        .map(|f| {
            f.claims
                .clone()
                .into_iter()
                .map(|(_, count)| match count == f.members {
                    true => 1,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn condense(raw: &str) -> CustomsForm {
    let mut claims: HashMap<char, i32> = HashMap::new();
    let mut members = 1;

    for c in raw.chars() {
        if c.is_alphabetic() {
            let count = claims.entry(c).or_insert(0);
            *count += 1;
        } else if c == '\n' {
            members += 1;
        }
    }

    CustomsForm { claims, members }
}

fn get_input(path: &str) -> Vec<CustomsForm> {
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
    println!("Individual Affirmatives {}", count_individual_affirmatives(forms.clone()));

    calibrate_part2();
    println!("Group Affirmatives {}", count_group_affirmatives(forms.clone()));
}
