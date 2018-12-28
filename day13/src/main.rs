use shared::Grid;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::io::prelude::*;

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> GenericResult<()> {
    let mut system = read_input()?;

    // system.print();

    part1(&mut system);

    Ok(())
}

fn read_input() -> GenericResult<System> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let w = lines.first().unwrap().len();
    let h = lines.len();

    let mut tracks = Grid::new(w.max(h), RefCell::new(TrackCell::new(' ')));
    let mut cars = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, &track) in line.iter().enumerate() {
            let (track, occupied) = match track {
                '<' | '>' | '^' | 'v' => {
                    cars.push(Car::new(x, y, track));
                    match track {
                        '<' | '>' => ('-', true),
                        '^' | 'v' => ('|', true),
                        _ => panic!(),
                    }
                }
                x => (x, false),
            };
            tracks[(x, y)] = RefCell::new(TrackCell { track, occupied });
        }
    }

    Ok(System { tracks, cars })
}

fn part1(sys: &mut System) {
    loop {
        // println!("tick!");
        match sys.tick() {
            Err(err) => {
                println!("{}", err);
                return;
            }
            Ok(_) => {
                // sys.print();
            }
        }
    }
}

struct System {
    tracks: Grid<RefCell<TrackCell>>,
    cars: Vec<Car>,
}

impl System {
    #![allow(dead_code)]
    fn print(&self) {
        for (x, y, cell) in self.tracks.iter() {
            if x == 0 {
                println!();
            }
            if cell.borrow().occupied {
                let car = self.cars.iter().find(|car| car.pos == Point(x, y)).unwrap();
                print!("{}", car.facing);
            } else {
                print!("{}", cell.borrow().track);
            }
        }

        println!();
    }

    fn tick(&mut self) -> Result<(), Crash> {
        self.cars.sort_unstable_by_key(|car| car.pos);

        for car in &mut self.cars {
            let Point(x, y) = car.pos;

            car.next_pos();

            let nc = &self.tracks[car.pos.into()];

            if nc.borrow().occupied {
                return Err(Crash {
                    pos: car.pos.into(),
                });
            }

            let nc_track = nc.borrow().track;
            if nc_track != '-' && nc_track != '|' {
                car.turn(nc_track);
            }

            let c = &self.tracks[(x, y)];
            c.borrow_mut().occupied = false;
            nc.borrow_mut().occupied = true;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct TrackCell {
    track: char,
    occupied: bool,
}

impl TrackCell {
    fn new(c: char) -> TrackCell {
        TrackCell {
            track: c,
            occupied: match c {
                '<' | '>' | '^' | 'v' => true,
                _ => false,
            },
        }
    }
}

struct Car {
    pos: Point,
    facing: char,
    turns: usize,
}

impl Car {
    fn new(x: usize, y: usize, facing: char) -> Car {
        Car {
            pos: Point(x, y),
            facing,
            turns: 0,
        }
    }

    fn next_pos(&mut self) {
        let Point(x, y) = self.pos;
        self.pos = match self.facing {
            '<' => Point(x - 1, y),
            '>' => Point(x + 1, y),
            '^' => Point(x, y - 1),
            'v' => Point(x, y + 1),
            _ => panic!("unknown car facing: {}", self.facing),
        }
    }

    fn turn_left(&mut self) {
        self.facing = match self.facing {
            '<' => 'v',
            '>' => '^',
            '^' => '<',
            'v' => '>',
            _ => panic!(),
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            '<' => '^',
            '>' => 'v',
            '^' => '>',
            'v' => '<',
            _ => panic!(),
        }
    }

    fn turn(&mut self, track: char) {
        if track == '+' {
            match self.pick_cross_direction() {
                Direction::Left => self.turn_left(),
                Direction::Straight => {}
                Direction::Right => self.turn_right(),
            };
        } else {
            match self.facing {
                '<' | '>' => match track {
                    '/' => self.turn_left(),
                    '\\' => self.turn_right(),
                    _ => panic!(
                        "unexpected state: car='{}' track='{}' @ ({})",
                        self.facing, track, self.pos
                    ),
                },
                '^' | 'v' => match track {
                    '/' => self.turn_right(),
                    '\\' => self.turn_left(),
                    _ => panic!(
                        "unexpected state: car='{}' track='{}' @ ({})",
                        self.facing, track, self.pos
                    ),
                },
                _ => panic!(
                    "unexpected state: car='{}' track='{}' @ ({})",
                    self.facing, track, self.pos
                ),
            };
        }
    }

    fn pick_cross_direction(&mut self) -> Direction {
        let d = match self.turns % 3 {
            0 => Direction::Left,
            1 => Direction::Straight,
            2 => Direction::Right,
            _ => panic!(),
        };
        self.turns += 1;
        d
    }
}

enum Direction {
    Left,
    Right,
    Straight,
}

#[derive(Debug)]
struct Crash {
    pos: Point,
}

impl fmt::Display for Crash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "crash: {}", self.pos)
    }
}

impl std::error::Error for Crash {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(usize, usize);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let Point(ref x, ref y) = self;
        let Point(ref ox, ref oy) = other;

        match y.cmp(oy) {
            Ordering::Equal => x.cmp(ox),
            ord => ord,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<(usize, usize)> for Point {
    fn into(self) -> (usize, usize) {
        (self.0, self.1)
    }
}
