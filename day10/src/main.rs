use std::collections::HashMap;
use std::fs;

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

// fn has_next(slice: Vec<i32>, value: i32) -> bool {
//     println!("Checking {:?} for adjacency to {}", slice.clone(), value);
//     slice
//         .iter()
//         .cloned()
//         .filter(|e| (value - *e).abs() < 4)
//         .collect::<Vec<_>>()
//         .len()
//         > 0
// }

fn chain_adapters(adapters: Vec<i32>) -> HashMap<i32, i32> {
    let mut ordered: Vec<i32> = adapters.iter().cloned().collect();
    ordered.sort_unstable();

    let mut differences: Vec<i32> = ordered.windows(2).map(|w| w[1] - w[0]).collect();

    differences.push((ordered[0] - 0).abs()); // account for the first transition
    differences.push(3); // account for the last transition

    let mut analysis = HashMap::new();

    for i in 0..4 {
        analysis.insert(
            i,
            differences.iter().copied().filter(|d| *d == i).count() as i32,
        );
    }

    analysis
}

fn part_1(path: &str) {
    let analysis = chain_adapters(load(path));
    
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

fn main() {
    println!("Day 10");
    println!("=============");
    part_1(r"c:\projects\github\advent-of-code-2020\data\day10.example.1.txt");
    part_1(r"c:\projects\github\advent-of-code-2020\data\day10.example.2.txt");
    part_1(r"c:\projects\github\advent-of-code-2020\data\day10.txt");
}
