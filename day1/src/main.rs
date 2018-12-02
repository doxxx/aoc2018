use std::collections::HashSet;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let changes = read_input()?;

    part1(&changes);
    part2(&changes);

    Ok(())
}

fn read_input() -> Result<Vec<i64>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.lines()
        .map(|line| line.parse::<i64>().expect("invalid number"))
        .collect())
}

fn part1(changes: &[i64]) {
    let mut freq = 0;
    for change in changes {
        freq += change;
    }
    println!("part1: {}", freq);
}

fn part2(changes: &[i64]) {
    let mut seen = HashSet::new();
    let mut freq = 0;
    loop {
        for change in changes {
            seen.insert(freq);
            freq += change;
            if seen.contains(&freq) {
                println!("part2: {}", freq);
                return;
            }
        }
        seen.insert(freq);
    }
}
