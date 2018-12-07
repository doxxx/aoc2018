use rayon::prelude::*;
use std::io::prelude::*;
use std::iter::FromIterator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let polymer = read_input()?;

    part1(&polymer);
    part2(&polymer);

    Ok(())
}

fn read_input() -> Result<Vec<char>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input.trim_end();
    Ok(input.chars().collect())
}

fn part1(polymer: &[char]) {
    let result = react(Vec::from(polymer));

    println!("{}", String::from_iter(result.iter()));
    println!("part1: {}", result.len());
}

fn react(mut input: Vec<char>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    loop {
        if input.len() == 0 {
            break;
        } else if input.len() == 1 {
            result.push(input.remove(0));
            break;
        }

        let a = input.remove(0);
        let b = input[0];
        if a.is_lowercase() != b.is_lowercase()
            && a.to_lowercase().next() == b.to_lowercase().next()
        {
            input.remove(0);
            if let Some(x) = result.pop() {
                input.insert(0, x);
            }
        } else {
            result.push(a);
        }
    }

    result
}

fn part2(polymer: &[char]) {
    let units: Vec<char> = (('a' as u8)..('z' as u8)).map(|b| b as char).collect();
    let shortest = units
        .par_iter()
        .map(|c| {
            let input: Vec<char> = Vec::from_iter(
                polymer
                    .iter()
                    .filter(|cc| *c != cc.to_lowercase().next().unwrap())
                    .map(|cc| *cc),
            );
            react(input).len()
        })
        .min()
        .unwrap();

    println!("part2: {}", shortest);
}
