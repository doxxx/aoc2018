use lazy_static::lazy_static;
use regex::Regex;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let (num_players, max_marble_value) = read_input()?;

    part1(num_players, max_marble_value);

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

fn part1(num_players: usize, max_marble_value: u64) {
    let mut circle: Vec<u64> = vec![0];
    let mut scores: Vec<u64> = vec![0; num_players];
    let mut current_player: usize = 0;
    let mut current_marble: usize = 0;

    // println!("[-] {:?}", circle);

    for current_marble_value in 1..=max_marble_value {
        if current_marble_value % 23 == 0 {
            let score = scores.get_mut(current_player).unwrap();
            *score += current_marble_value;
            let removal_point = if current_marble >= 7 {
                current_marble - 7
            } else {
                while current_marble < 7 {
                    current_marble += circle.len()
                }
                current_marble - 7
            };
            *score += circle.remove(removal_point);
            current_marble = removal_point;
        } else {
            let mut insertion_point = current_marble + 2;
            while insertion_point > circle.len() {
                insertion_point -= circle.len();
            }
            circle.insert(insertion_point, current_marble_value);
            current_marble = insertion_point;
        }
        // println!("[{}] {:?}", current_player +1, circle);
        current_player = (current_player + 1) % num_players;
    }

    //println!("scores: {:?}", scores);

    let result = scores.iter().max().unwrap();
    println!("max score: {}", result);
}
