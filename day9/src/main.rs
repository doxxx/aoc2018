use lazy_static::lazy_static;
use regex::Regex;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let (num_players, max_marble_value) = read_input()?;

    play(num_players, max_marble_value);

    Ok(())
}

fn read_input() -> Result<(usize, u64)> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(parse_input(&input))
}

fn parse_input(s: &str) -> (usize, u64) {
    lazy_static! {
        static ref re: Regex =
            Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    }

    let caps = re.captures(s).unwrap();

    (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

fn play(num_players: usize, max_marble_value: u64) {
    let mut circle = Circle::new();
    let mut scores: Vec<u64> = vec![0; num_players];
    let mut current_player: usize = 0;

    // println!("[-] {}", circle);

    for current_marble_value in 1..=max_marble_value {
        if current_marble_value % 23 == 0 {
            let score = scores.get_mut(current_player).unwrap();
            *score += current_marble_value;
            *score += circle.remove_marble(circle.counter_clockwise(7));
        } else {
            circle.insert_marble_after(circle.clockwise(1), current_marble_value);
        }
        // println!("[{}] {}", current_player + 1, circle);
        current_player = (current_player + 1) % num_players;
    }

    //println!("scores: {:?}", scores);

    let result = scores.iter().max().unwrap();
    println!("max score: {}", result);
}

struct Circle {
    marbles: Vec<Marble>,
    current: MarbleID,
    first: MarbleID,
}

type MarbleID = usize;
type MarbleValue = u64;

#[derive(Clone)]
struct Marble {
    value: MarbleValue,
    prev: MarbleID,
    next: MarbleID,
}

impl Circle {
    fn new() -> Circle {
        let mut c = Circle {
            marbles: Vec::new(),
            current: 0,
            first: 0,
        };
        c.add_marble(0);
        c
    }

    fn insert_marble_after(&mut self, i: MarbleID, value: MarbleValue) -> MarbleID {
        let after = self.marbles[i].next;
        let new = self.add_marble(value);
        self.marbles[new].prev = i;
        self.marbles[new].next = after;
        self.marbles[i].next = new;
        self.marbles[after].prev = new;
        self.current = new;
        new
    }

    fn remove_marble(&mut self, i: MarbleID) -> MarbleValue {
        let Marble { value, prev, next } = self.marbles[i];
        self.marbles[prev].next = next;
        self.marbles[next].prev = prev;
        self.current = next;
        if i == self.first {
            self.first = self.current;
        }
        value
    }

    fn clockwise(&self, mut steps: usize) -> MarbleID {
        let mut i = self.current;
        while steps > 0 {
            steps -= 1;
            i = self.marbles[i].next;
        }
        i
    }

    fn counter_clockwise(&self, mut steps: usize) -> MarbleID {
        let mut i = self.current;
        while steps > 0 {
            steps -= 1;
            i = self.marbles[i].prev;
        }
        i
    }

    fn add_marble(&mut self, value: MarbleValue) -> MarbleID {
        let i = self.marbles.len();
        self.marbles.push(Marble::new(value));
        i
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut i = self.first;

        loop {
            let m = &self.marbles[i];
            if i == self.current {
                write!(f, "({})", m.value)?;
            } else {
                write!(f, " {} ", m.value)?;
            }
            i = m.next;
            if i == self.first {
                break;
            }
        }

        Ok(())
    }
}

impl Marble {
    fn new(value: MarbleValue) -> Marble {
        Marble {
            value,
            prev: 0,
            next: 0,
        }
    }
}
