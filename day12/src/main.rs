use std::fmt;
use std::fs;
use substring::Substring;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Action {
    direction: Direction,
    amount: i32,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0:?} for {1}", self.direction, self.amount)
    }
}

#[derive(Clone, Copy)]
struct Ship {
    bearing: Direction,
    x: i32,
    y: i32,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            bearing: Direction::East,
            x: 0,
            y: 0,
        }
    }

    pub fn distance(&self, ship: Ship) -> i32 {
        (self.x - ship.x).abs() + (self.y - ship.y).abs()
    }

    pub fn travel(&self, action: &Action) -> Ship {
        match action.direction {
            Direction::North => Ship {
                bearing: self.bearing,
                y: self.y + action.amount,
                x: self.x,
            },
            Direction::South => Ship {
                bearing: self.bearing,
                y: self.y - action.amount,
                x: self.x,
            },
            Direction::East => Ship {
                bearing: self.bearing,
                y: self.y,
                x: self.x + action.amount,
            },
            Direction::West => Ship {
                bearing: self.bearing,
                y: self.y,
                x: self.x - action.amount,
            },
            Direction::Left => Ship {
                bearing: self.turn(action),
                y: self.y,
                x: self.x,
            },
            Direction::Right => Ship {
                bearing: self.turn(action),
                y: self.y,
                x: self.x,
            },
            Direction::Forward => match self.bearing {
                Direction::North => Ship {
                    bearing: self.bearing,
                    y: self.y + action.amount,
                    x: self.x,
                },
                Direction::South => Ship {
                    bearing: self.bearing,
                    y: self.y - action.amount,
                    x: self.x,
                },
                Direction::East => Ship {
                    bearing: self.bearing,
                    y: self.y,
                    x: self.x + action.amount,
                },
                Direction::West => Ship {
                    bearing: self.bearing,
                    y: self.y,
                    x: self.x - action.amount,
                },
                x => panic!(
                    "Only cardinal directions are supported. {:?} is not a cardinal direction",
                    x
                ),
            },
        }
    }
    fn turn(&self, action: &Action) -> Direction {
        let headings = match self.bearing {
            Direction::North => [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            Direction::East => [
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::North,
            ],
            Direction::South => [
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::East,
            ],
            Direction::West => [
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::South,
            ],
            x => panic!(
                "Only cardinal directions are supported. {:?} is not a cardinal direction",
                x
            ),
        };

        let index = (action.amount.rem_euclid(360) / 90) as usize;
        if action.direction == Direction::Left {
            headings[4 - index]
        } else if action.direction == Direction::Right {
            headings[index]
        } else {
            panic!("{:?} is not a relational position change", action.direction)
        }
    }
}

fn load(path: &str) -> Vec<Action> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Uable to load the file {}", path))
        .lines()
        .map(|l| {
            let direction = match l.substring(0, 1) {
                "N" => Direction::North,
                "S" => Direction::South,
                "E" => Direction::East,
                "W" => Direction::West,
                "L" => Direction::Left,
                "R" => Direction::Right,
                "F" => Direction::Forward,
                x => panic!("Found unknown direction {}", x),
            };
            let amount = l
                .substring(1, l.len())
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Unable to parse the magnitude from {}", l));

            Action { direction, amount }
        })
        .collect()
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0:?} at ({1},{2})", self.bearing, self.x, self.y)
    }
}

fn embark(ship: Ship, journey: &[Action]) -> Ship {
    let mut ship2 = ship.clone();

    for leg in journey {
        print!("{} -> {} -> ", ship2, leg);
        ship2 = ship2.travel(leg);
        println!("{}", ship2);
    }

    ship2
}

fn part1(journey: &[Action]) -> i32 {
    let initial = Ship::new();
    let ship = embark(initial, journey);
    initial.distance(ship)
}

fn main() {
    println!("Day 12");
    println!("=============");
    calibrate_part1();

    println!(
        "Part 1 - {} distance traveled",
        part1(&load(r"c:\projects\github\advent-of-code-2020\data\day12.txt"))
    );
}

fn calibrate_part1() {
    println!("Calibration -- Part 1");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day12.example.txt",
        25,
    )];

    for input in inputs.iter() {
        let value = part1(&load(input.0));
        println!(
            "{0} => {1} [{2}]",
            input.0,
            value,
            match value == input.1 {
                true => "SUCCESS",
                _ => panic!("FAILED.  Expected {} but calculated {}", input.1, value),
            }
        );
    }
}
