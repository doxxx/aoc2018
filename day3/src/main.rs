use lazy_static::lazy_static;
use regex::Regex;
use std::io::prelude::*;
use std::collections::HashMap;
use shared::Grid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let claims = read_input()?;

    part1(&claims);
    part2(&claims);

    Ok(())
}

struct Claim {
    pub id: usize,
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

fn read_input() -> Result<Vec<Claim>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.lines().map(|s| parse_claim(s)).collect())
}

fn parse_claim(s: &str) -> Claim {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }

    let caps = RE.captures(s).unwrap();

    Claim {
        id: caps[1].parse().unwrap(),
        left: caps[2].parse().unwrap(),
        top: caps[3].parse().unwrap(),
        width: caps[4].parse().unwrap(),
        height: caps[5].parse().unwrap(),
    }
}

fn part1(claims: &[Claim]) {
    let mut grid = Grid::new(1000, 0);

    for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                grid[(x, y)] += 1;
            }
        }
    }

    let num_overlaps = grid.cells.iter().filter(|c| **c >= 2).count();

    println!("part1: {}", num_overlaps);
}

fn part2(claims: &[Claim]) {
    let mut grid: Grid<Vec<usize>> = Grid::new(1000, Vec::new());

    for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                grid[(x, y)].push(claim.id);
            }
        }
    }

    let mut overlaps: HashMap<usize,bool> = HashMap::new();

    for ids in grid.cells {
        if ids.len() > 1 {
            for id in ids {
                overlaps.insert(id, true);
            }
        }
    }

    for claim in claims {
        if !overlaps.contains_key(&claim.id) {
            println!("part2: {}", claim.id);
            return;
        }
    }
}
