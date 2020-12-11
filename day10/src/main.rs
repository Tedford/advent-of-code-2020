use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::time::{Instant};

fn load(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Unable to load the input file {}", path))
        .lines()
        .map(|l| {
            l.parse::<i32>()
                .unwrap_or_else(|_| panic!("Unable to parse {}", l))
        })
        .collect()
}

fn chain_adapters(adapters: Vec<i32>) -> Vec<i32> {
    let mut ordered: Vec<i32> = adapters.to_vec();
    ordered.sort_unstable();
    ordered.insert(0, 0); // account for initial voltage
    ordered.push(ordered[ordered.len() - 1] + 3); // account for device draw

    ordered
}

fn aggregate_joltage_drop(adapters: Vec<i32>) -> HashMap<i32, i32> {
    let differences = calculate_differences(&adapters);

    let mut analysis = HashMap::new();

    for i in 0..4 {
        analysis.insert(
            i,
            differences.iter().copied().filter(|d| *d == i).count() as i32,
        );
    }

    analysis
}

fn calculate_differences(adapters: &[i32]) -> Vec<i32> {
    adapters.windows(2).map(|w| w[1] - w[0]).collect()
}

fn is_valid(adapters: &[i32]) -> bool {
    calculate_differences(adapters)
        .iter()
        .copied()
        .find(|d| *d > 3)
        .is_none()
}

fn hash(array: &[i32]) -> String {
    let mut out = String::new();

    for n in array {
        let _ = write!(&mut out, "{}", n);
    }

    out
}

fn discover_permutations(adapters: &[i32]) -> i32 {
    let mut permutations: HashSet<String> = HashSet::new();
    let mut threads = vec![];

    for i in 1 .. adapters.len() -2 {
        let mut c = adapters.clone().to_vec();
        c.remove(i);
        threads.push(thread::spawn(move || -> HashSet<String>{
            let mut p : HashSet<String> = HashSet::new();
            permutate(c,&mut p);
            p
        }));
    }

    for t in threads {
        let found = t.join().unwrap();
        for hash in found {
            permutations.insert(hash);
        }
    }

    permutations.len() as i32 + 1
}

fn permutate(adapters: Vec<i32>, permutations: &mut HashSet<String>) {
    match is_valid(&adapters) {
        true => {
            let length = adapters.len();
            let hash = hash(&adapters);

            if !permutations.contains(&hash) {
                permutations.insert(hash);
                for i in 1..length - 2 {
                    let mut slice = adapters.clone();
                    slice.remove(i);
                    permutate(slice, permutations);
                }
            }
        }
        false => (),
    }
}

fn part_1(path: &str) {
    let analysis = aggregate_joltage_drop(chain_adapters(load(path)));
    println!("{} produced {:?}", path, analysis);

    println!(
        "Joltage Difference: {}",
        analysis
            .get(&1)
            .unwrap_or_else(|| panic!("Unable to find joltage by 1 difference"))
            * analysis
                .get(&3)
                .unwrap_or_else(|| panic!("Unable to find joltage by 3 difference"))
    );
}

fn part_2(path: &str) {
    let now = Instant::now();
    let permutations = discover_permutations(&chain_adapters(load(path)));
    println!(
        "{} produced {:?} variaions in {:#?}",
        path,
        permutations,
        now.elapsed()
    );
}

fn main() {
    println!("Day 10");
    println!("=============");
    part_1(r"c:\projects\github\advent-of-code-2020\data\day10.example.1.txt");
    part_1(r"c:\projects\github\advent-of-code-2020\data\day10.example.2.txt");
    part_1(r"c:\projects\github\advent-of-code-2020\data\day10.txt");

    part_2(r"c:\projects\github\advent-of-code-2020\data\day10.example.1.txt");
    part_2(r"c:\projects\github\advent-of-code-2020\data\day10.example.2.txt");
    part_2(r"c:\projects\github\advent-of-code-2020\data\day10.txt");
}
