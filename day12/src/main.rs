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

impl Action {
    pub fn normalize(&self) -> i32 {
        match self.direction {
            Direction::South | Direction::West => self.amount * -1,
            _ => self.amount,
        }
    }
}

#[derive(Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    pub fn distance(&self, loc: Location) -> i32 {
        (self.x - loc.x).abs() + (self.y - loc.y).abs()
    }

    pub fn add_x(&self, x: i32) -> Location {
        Location {
            y: self.y,
            x: self.x + x,
        }
    }

    pub fn add_y(&self, y: i32) -> Location {
        Location {
            y: self.y + y,
            x: self.x,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
struct Ship {
    bearing: Direction,
    location: Location,
    waypoint: Location,
}

impl Ship {
    pub fn new(waypoint: Location) -> Self {
        Ship {
            bearing: Direction::East,
            location: Location { x: 0, y: 0 },
            waypoint,
        }
    }

    pub fn waypoint_travel(&self, action: &Action) -> Ship {
        match action.direction {
            Direction::North | Direction::South => Ship {
                waypoint: self.waypoint.add_y(action.normalize()),
                ..*self
            },
            Direction::East | Direction::West => Ship {
                waypoint: self.waypoint.add_x(action.normalize()),
                ..*self
            },
            Direction::Left => self.change_bearing(action.amount),
            Direction::Right => self.change_bearing(-action.amount),
            Direction::Forward => Ship {
                location: Location {
                    x: self.location.x + self.waypoint.x * action.amount,
                    y: self.location.y + self.waypoint.y * action.amount,
                },
                ..*self
            },
        }
    }

    pub fn direct_travel(&self, action: &Action) -> Ship {
        match action.direction {
            Direction::North | Direction::South => Ship {
                location: self.location.add_y(action.normalize()),
                ..*self
            },
            Direction::East | Direction::West => Ship {
                location: self.location.add_x(action.normalize()),
                ..*self
            },
            Direction::Left => Ship {
                bearing: self.turn(action),
                ..*self
            },
            Direction::Right => Ship {
                bearing: self.turn(action),
                ..*self
            },
            Direction::Forward => match self.bearing {
                Direction::North => Ship {
                    location: self.location.add_y(action.amount),
                    ..*self
                },
                Direction::South => Ship {
                    location: self.location.add_y(action.amount * -1),
                    ..*self
                },
                Direction::East => Ship {
                    location: self.location.add_x(action.amount),
                    ..*self
                },
                Direction::West => Ship {
                    location: self.location.add_x(action.amount * -1),
                    ..*self
                },
                x => panic!(
                    "Only cardinal directions are supported. {:?} is not a cardinal direction",
                    x
                ),
            },
        }
    }

    fn change_bearing(&self, degree: i32) -> Ship {
        let mut ship = self.clone();

        for _ in 0 .. (360 + degree).rem_euclid(360) / 90
        {
            ship.waypoint = if degree > 0 {
                Location { x: -ship.waypoint.y, y: ship.waypoint.x}
            }
            else {
                Location { x: ship.waypoint.y, y: -ship.waypoint.x}
            }
        }

        // ship.waypoint = match (360 + degree).rem_euclid(360) {
        //     90 => Location {
        //         x: -ship.waypoint.y,
        //         y: ship.waypoint.x,
        //     },
        //     180 => Location {
        //         x: -ship.waypoint.y,
        //         y: -ship.waypoint.x,
        //     },
        //     270 => Location {
        //         x: ship.waypoint.y,
        //         y: -ship.waypoint.x,
        //     },
        //     _ => ship.waypoint,
        // };

        ship
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
        write!(
            f,
            "{:?} at {} heading {}",
            self.bearing, self.location, self.waypoint
        )
    }
}

fn embark<T>(ship: Ship, journey: &[Action], travel: T) -> Ship
where
    T: Fn(&Ship, &Action) -> Ship,
{
    let mut ship2 = ship.clone();

    for leg in journey {
        print!("{} -> {} -> ", ship2, leg);
        ship2 = travel(&ship2, leg);
        println!("{}", ship2);
    }

    ship2
}

fn part1(journey: &[Action]) -> i32 {
    let initial = Ship::new(Location { x: 0, y: 0 });
    let ship = embark(initial, journey, Ship::direct_travel);
    initial.location.distance(ship.location)
}

fn part2(journey: &[Action]) -> i32 {
    let initial = Ship::new(Location { x: 10, y: 1 });
    let ship = embark(initial, journey, Ship::waypoint_travel);
    initial.location.distance(ship.location)
}

fn main() {
    println!("Day 12");
    println!("=============");
    calibrate_part1();

    println!(
        "\n\nPart 1 - {} distance traveled",
        part1(&load(
            r"c:\projects\github\advent-of-code-2020\data\day12.txt"
        ))
    );

    calibrate_part2();

    // 24119 too low
    // 42451 too high
    println!(
        "\n\nPart 2 - {} distance traveled",
        part2(&load(
            r"c:\projects\github\advent-of-code-2020\data\day12.txt"
        ))
    );
}

fn calibrate_part1() {
    println!("\n\nCalibration -- Part 1");
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
    println!("\n\n");
}

fn calibrate_part2() {
    println!("\n\nCalibration -- Part 2");
    let inputs = [(
        r"c:\projects\github\advent-of-code-2020\data\day12.example.txt",
        286,
    )];

    for input in inputs.iter() {
        let value = part2(&load(input.0));
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
    println!("\n\n");
}
