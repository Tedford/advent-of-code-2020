use std::fs;

#[allow(dead_code)]
struct Credential {
    min: usize,
    max: usize,
    token: char,
    password: String,
    policy1: bool,
    policy2: bool,
}

fn parse_record(input: &str) -> Credential {
    let mut iter = input.split_whitespace();
    let range: Vec<&str> = iter.next().unwrap().split("-").collect();
    let min = range[0].parse::<usize>().unwrap();
    let max = range[1].parse::<usize>().unwrap();
    let token = iter.next().unwrap().chars().next().unwrap();
    let password = iter.next().unwrap().to_string();
    let count = password.chars().filter(|c| *c == token).count();

    let mut matches: i32 = 0;
    if password.chars().nth(min - 1).unwrap() == token {
        matches = matches + 1;
    }
    if password.chars().nth(max - 1).unwrap() == token {
        matches = matches + 1;
    }

    Credential {
        min: min,
        max: max,
        token: token,
        password: password,
        policy1: count >= min && count <= max,
        policy2: matches == 1,
    }
}

fn load_credentials(path: &str) -> Vec<Credential> {
    fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|x| parse_record(x))
        .collect()
}

fn print_report(path: &str) {
    let parts: Vec<&str> = path.split("\\").collect();
    let filename = parts[parts.len() - 1];

    println!("{}", filename);
    let credentials = load_credentials(path);
    println!(
        "\t{0} of {2} passwords are valid with Policy 1\n\t{1} of {2} are valid with Policy 2",
        credentials.iter().filter(|c| c.policy1).count(),
        credentials.iter().filter(|c| c.policy2).count(),
        credentials.len()
    )
}

fn main() {
    println!("Day 2");
    println!("=============");

    print_report("c:\\projects\\github\\advent-of-code-2020\\data\\day2.example.txt");
    print_report("c:\\projects\\github\\advent-of-code-2020\\data\\day2.txt");
}
