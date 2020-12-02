use std::fs;

#[allow(dead_code)]
struct Credential {
    min: usize,
    max: usize,
    token: char,
    password: String,
    valid: bool,
}

fn parse_record(input: &str) -> Credential {
    let mut iter = input.split_whitespace();
    let range: Vec<&str> = iter.next().unwrap().split("-").collect();
    let min = range[0].parse::<usize>().unwrap();
    let max = range[1].parse::<usize>().unwrap();

    let token = iter.next().unwrap().chars().next().unwrap();
    let password = iter.next().unwrap();

    let count = password.chars().filter(|c| *c == token).count();

    Credential {
        min: min,
        max: max,
        token: token,
        password: password.to_string(),
        valid: count >= min && count <= max,
    }
}

fn load_credentials(path: &str) -> Vec<Credential> {
    let credentials: Vec<Credential> = fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|x| parse_record(x))
        .collect();
    credentials
}

fn main() {
    println!("Day 2");
    println!("=============");
    println!("Part1");
    println!("=======");

    println!("day2.example.txt");
    let credentials =
        load_credentials("c:\\projects\\github\\advent-of-code-2020\\data\\day2.example.txt");
    println!(
        "\t{} of {} passwords are valid",
        credentials.iter().filter(|c| c.valid).count(),
        credentials.len()
    );

    println!("day2.txt");
    let credentials = load_credentials("c:\\projects\\github\\advent-of-code-2020\\data\\day2.txt");
    println!(
        "\t{} of {} passwords are valid",
        credentials.iter().filter(|c| c.valid).count(),
        credentials.len()
    );
}
