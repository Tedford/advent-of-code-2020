use std::collections::HashSet;
use std::fmt;
use std::fs;
use substring::Substring;

#[derive(Debug, Clone)]
struct Instruction {
    operand: String,
    positive: bool,
    offset: i32,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {:5}",
            self.operand,
            if self.positive { "+" } else { "-" },
            self.offset
        )
    }
}

struct Processor {
    accumulator: i32,
    current: i32,
    processed: HashSet<i32>,
    instructions: Vec<Instruction>,
}

impl Processor {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Processor {
            accumulator: 0,
            current: 0,
            processed: HashSet::new(),
            instructions,
        }
    }

    pub fn next(&mut self) -> bool {
        let instruction = self.instructions[self.current as usize].clone();

        println!(
            "[{0:4}] {1:5} | {2}",
            self.current,
            self.accumulator,
            instruction.clone()
        );

        let first = self.processed.contains(&self.current);
        self.processed.insert(self.current);

        self.current = self.current
            + match instruction.operand.as_str() {
                "acc" => {
                    self.accumulator = self.accumulator
                        + (instruction.offset * if instruction.positive { 1 } else { -1 });
                    1
                }
                "jmp" => instruction.offset * if instruction.positive { 1 } else { -1 },
                _ => 1,
            };

        first
    }
}

impl fmt::Display for Processor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", self.accumulator)
    }
}

fn parse(raw: &str) -> Instruction {
    let operand = raw.substring(0, 3).to_string();
    let positive = match raw.substring(4, 5) {
        "+" => true,
        _ => false,
    };
    let offset = raw.substring(5, raw.len()).parse::<i32>().unwrap();

    Instruction {
        operand,
        positive,
        offset,
    }
}

fn load(path: &str) -> Vec<Instruction> {
    fs::read_to_string(path)
        .expect(path)
        .lines()
        .map(|l| parse(l))
        .collect()
}

fn calibrate_part1() {
    println!("Calibration");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day8.example.txt",
        5,
    )];

    for input in inputs.iter() {
        let mut processor = Processor::new(load(input.0));

        let value = part1(&mut processor);
        println!(
            "{0} => {1} [{2}]",
            input.0,
            value,
            match value == input.1 {
                true => "SUCCESS",
                _ => "FAILED",
            }
        );
    }
}

fn part1(processor: &mut Processor) -> i32 {
    let mut last = 0;
    while !processor.next() {
        last = processor.accumulator;
    }

    last
}

fn main() {
    println!("Day 7");
    println!("=============");
    let code = load(r"c:\projects\github\advent-of-code-2020\data\day8.txt");

    calibrate_part1();

    let mut processor = Processor::new(code);

    println!("Part 1 {}", part1(&mut processor));
}
