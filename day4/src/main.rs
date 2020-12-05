#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref HAIRCOLOR_EX: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    static ref EYECOLOR_EX: Regex = Regex::new("amb|blu|brn|gry|grn|hzl|oth").unwrap();
    static ref PASSPORT_ID_EX: Regex = Regex::new("[0-9]{9}").unwrap();
    static ref HEIGHT_EX: Regex = Regex::new("(?P<size>[0-9]+)(?P<unit>in|cm)").unwrap();
}

fn parse_credentials(path: &str) -> Vec<HashMap<String, String>> {
    let mut credentials = vec![HashMap::<String, String>::new()];
    for l in fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
    {
        if l == "" {
            credentials.push(HashMap::<String, String>::new());
        } else {
            for token in l.split_whitespace() {
                let current = credentials.last_mut().unwrap();
                let parts = token.split(':').collect::<Vec<&str>>();
                current.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }
    credentials
}

fn in_range(value: Option<&String>, min: i32, max: i32) -> bool {
    match value {
        Some(value) => {
            let v = value.parse::<i32>().unwrap();
            v > min && v < max
        }
        _ => false,
    }
}

fn matches(value: Option<&String>, criteria: &str) -> bool {
    match value {
        Some(value) => match criteria {
            "hcl" => HAIRCOLOR_EX.is_match(value),
            "ecl" => EYECOLOR_EX.is_match(value),
            "pid" => PASSPORT_ID_EX.is_match(value),
            _ => false,
        },
        _ => false,
    }
}

fn is_valid(credential: &HashMap<String, String>) -> bool {
    in_range(credential.get("byr"), 1919, 2003)
        && in_range(credential.get("iyr"), 2009, 2021)
        && in_range(credential.get("eyr"), 2019, 2031)
        && match credential.get("hgt") {
            Some(value) => {
                let captures = HEIGHT_EX.captures(value);
                match captures {
                    Some(capture) => match &capture[2] {
                        "in" => {
                            let v = capture[1].parse::<i32>().unwrap();
                            v > 58 && v < 77
                        }
                        "cm" => {
                            let v = capture[1].parse::<i32>().unwrap();
                            v > 149 && v < 194
                        }
                        _ => false,
                    },
                    _ => false,
                }
            }
            _ => false,
        }
        && matches(credential.get("hcl"), "hcl")
        && matches(credential.get("ecl"), "ecl")
        && matches(credential.get("pid"), "pid")
}

fn has_fields(credential: &HashMap<String, String>) -> bool {
    match credential.len() {
        8 => true,
        7 => !credential.contains_key("cid"),
        _ => false,
    }
}

fn main() {
    println!("Day 4");
    println!("=============");

    let files = [
        "C:\\Projects\\GitHub\\advent-of-code-2020\\Data\\day4.example.txt",
        "C:\\Projects\\GitHub\\advent-of-code-2020\\Data\\day4.txt",
    ];

    println!("Part 1");
    for file in files.iter() {
        let count = parse_credentials(file)
            .into_iter()
            .filter(|h| has_fields(h))
            .count();

        println!("\t{0}\t{1}", file, count);
    }

    let calibration_files = [
        "C:\\Projects\\GitHub\\advent-of-code-2020\\Data\\day4.part2.invalid.txt",
        "C:\\Projects\\GitHub\\advent-of-code-2020\\Data\\day4.part2.valid.txt",
    ];

    println!("Part 2");
    println!("Calibration");
    for file in calibration_files.iter() {
        let count = parse_credentials(file)
            .into_iter()
            .filter(|h| is_valid(h))
            .count();

        println!("\t{0}\t{1}", file, count);
    }

    // 103 too low
    // 128 too high
    println!("Execution");
    for file in [files[1]].iter() {
        let count = parse_credentials(file)
            .into_iter()
            .filter(|h| has_fields(h) && is_valid(h))
            .count();
        println!("\t{0}\t{1}", file, count);
    }
}
