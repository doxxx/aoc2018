use std::io::prelude::*;
use std::iter::FromIterator;

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> GenericResult<()> {
    let input = read_input()?;

    part1(input);

    Ok(())
}

fn read_input() -> GenericResult<usize> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.trim().parse().unwrap())
}

fn part1(num_recipes: usize) {
    let mut scoreboard = Scoreboard(vec![3, 7]);
    let mut elves = [0, 1];

    loop {
        let sum: u8 = elves.iter().map(|&e| scoreboard.score(e)).sum();
        let new_scores = if sum >= 10 {
            vec![(sum / 10) % 10, sum % 10]
        } else {
            vec![sum]
        };

        scoreboard.add(&new_scores);

        elves
            .iter_mut()
            .for_each(|e| *e = scoreboard.step_forward(*e, 1 + scoreboard.score(*e) as usize));

        // println!("{}", scores_to_string(scoreboard.all_scores()));

        if scoreboard.all_scores().len() >= num_recipes + 10 {
            break;
        }
    }

    println!(
        "result: {}",
        scores_to_string(&scoreboard.all_scores()[num_recipes..num_recipes + 10])
    );
}

fn scores_to_string(scores: &[u8]) -> String {
    String::from_iter(scores.iter().map(|&s| ('0' as u8 + s) as char))
}

struct Scoreboard(Vec<u8>);

impl Scoreboard {
    fn score(&self, index: usize) -> u8 {
        self.0[index]
    }

    fn add(&mut self, scores: &[u8]) {
        self.0.extend_from_slice(scores);
    }

    fn step_forward(&self, start: usize, count: usize) -> usize {
        (start + count) % self.0.len()
    }

    fn all_scores(&self) -> &[u8] {
        &self.0
    }
}
