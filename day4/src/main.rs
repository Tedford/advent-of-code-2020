use std::collections::HashMap;
use std::fs;

// fn injest_tokens(hash: &HashMap<String,String>, line: &str) {
//     for token in line.split_whitespace().map(|t| t.split(':')) {
//         println!("{:?}", token);
//         hash.insert("left".to_string(), "right".to_string());
//     }
// }

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

fn main() {
    println!("Day 4");
    println!("=============");

    let files = [
        "C:\\Projects\\GitHub\\advent-of-code-2020\\Data\\day4.example.txt",
        "C:\\Projects\\GitHub\\advent-of-code-2020\\Data\\day4.txt",
    ];

    for file in files.iter() {
        let credentials = parse_credentials(file);
        let count = credentials
            .into_iter()
            .filter(|h| {
                h.len() == 8
                    || (h.len() == 7
                        && match h.get("cid") {
                            None => true,
                            _ => false,
                        })
            })
            .count();

        println!("\t{0}\t{1}", file, count);
    }
    // .fold((Vec::<Vec<String>>::new(), ), |acc, l| {
    // .fold((Vec::<HashMap<String, String>>::new(), None), |acc, l| {
    // // .fold((credentials, None), |acc, l| {
    // if l == "" {
    //     (acc.0, None)
    // } else {
    //     let mut creds = acc.0.clone();
    //     let mut current = match acc.1 {
    //         None => {
    //             let mut c = HashMap::new();
    //             creds.push(c);
    //             c
    //         }
    //         _ => acc.1.unwrap(),
    //     };

    //     (creds, Some(current))
    // }
    //     acc
    // });
}
