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

#[derive(Copy,Clone)]
enum State {
    Uninitalized,
    Running,
    Errored,
    Completed,
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
    instructions: Vec<Instruction>,
    processed: HashSet<i32>,
    state: State,
}

impl Processor {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Processor {
            accumulator: 0,
            current: 0,
            processed: HashSet::new(),
            state: State::Uninitalized,
            instructions,
        }
    }

    pub fn next(&mut self) -> Option<i32> {
        if self.current < self.instructions.len() as i32 {
            if self.processed.contains(&self.current) {
                self.state = State::Errored;
                None
            } else {
                self.state = State::Running;
                self.processed.insert(self.current);
                let ip = Some(self.current);
                let instruction = self.instructions[self.current as usize].clone();
                println!(
                    "[{0:4}] {1:5} | {2}",
                    self.current,
                    self.accumulator,
                    instruction.clone()
                );
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
                ip
            }
        } else {
            self.state = State::Completed;
            None
        }
    }

    pub fn run(&mut self) -> (State, i32) {
        while let Some(_) = self.next() {}
        (self.state, self.accumulator)
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
    println!("Calibration -- Part 1");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day8.example.1.txt",
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

fn calibrate_part2() {
    println!("Calibration -- Part 2");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day8.example.1.txt",
        8,
    )];

    for input in inputs.iter() {
        let value = part2(&load(input.0));
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
    let mut last: i32 = 0;

    while let Some(_) = processor.next() {
        last = processor.accumulator;
    }

    last
}

fn part2(code: &Vec<Instruction>) -> i32 {
    let mut current: usize = 0;
    let mut value: Option<i32> = None;
    let count = code.clone().len();

    while value == None && current < count {
        let mut new_code = code.clone();
        value = match new_code[current].operand.as_str() {
            "jmp" => {
                new_code[current].operand = "nop".to_string();
                let mut processor = Processor::new(new_code);
                let result = processor.run();

                match result.0 {
                    State::Completed => Some(result.1),
                    _ => None,
                }
            }
            "nop" => {
                new_code[current].operand = "jmp".to_string();
                let mut processor = Processor::new(new_code);
                let result = processor.run();

                match result.0 {
                    State::Completed => Some(result.1),
                    _ => None,
                }
            }
            _ => None,
        };
        current = current + 1;
    }

    match value {
        None => panic!("Unable to find a solution"),
        Some(x) => x,
    }
}

fn main() {
    println!("Day 8");
    println!("=============");
    let code = load(r"c:\projects\github\advent-of-code-2020\data\day8.txt");

    calibrate_part1();

    let mut processor = Processor::new(code.clone());

    println!("Part 1 {}", part1(&mut processor));

    calibrate_part2();

    println!("Part 2 {}",part2(&code.clone()));

}
