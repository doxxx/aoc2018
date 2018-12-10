use std::i32;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let coordinates = read_input()?;

    part1(&coordinates);
    part2(&coordinates);

    Ok(())
}

fn read_input() -> Result<Vec<Coordinate>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.trim_end().lines().map(parse_coordinate).collect())
}

fn parse_coordinate(s: &str) -> Coordinate {
    let mut i = s.split(",");
    let x: i32 = i.next().unwrap().trim().parse().unwrap();
    let y: i32 = i.next().unwrap().trim().parse().unwrap();

    Coordinate { x, y }
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn part1(coordinates: &[Coordinate]) {
    let (gw, gh) = grid_size(coordinates);
    let mut closest_counts = vec![0; coordinates.len()];

    for y in 0..gh {
        for x in 0..gw {
            let p = Coordinate { x, y };
            let distances: Vec<i32> = coordinates.iter().map(|c| c.distance(&p)).collect();
            if let Some(i) = closest_coordinate(&distances) {
                if x == 0 || y == 0 || x == (gw - 1) || y == (gh - 1) {
                    closest_counts[i] = i32::MIN;
                } else {
                    closest_counts[i] += 1;
                }
            } else {
                continue;
            }
        }
    }

    let result = closest_counts.iter().max().unwrap();

    println!("part1: {}", result);
}

fn grid_size(coordinates: &[Coordinate]) -> (i32, i32) {
    (
        coordinates.iter().map(|c| c.x).max().unwrap() + 1,
        coordinates.iter().map(|c| c.y).max().unwrap() + 1,
    )
}

fn closest_coordinate(distances: &[i32]) -> Option<usize> {
    let mut distances: Vec<(usize, i32)> = distances.iter().map(|&d| d).enumerate().collect();
    distances.sort_by_key(|(_, d)| *d);
    let (index, distance) = distances.remove(0);
    if distances.iter().any(|(_, d)| *d == distance) {
        None
    } else {
        Some(index)
    }
}

fn part2(coordinates: &[Coordinate]) {
    let (gw, gh) = grid_size(coordinates);
    let mut count = 0;

    for y in 0..gh {
        for x in 0..gw {
            let p = Coordinate { x, y };
            let sum_distance: i32 = coordinates.iter().map(|c| p.distance(c)).sum();
            if sum_distance < 10000 {
                count += 1;
            }
        }
    }

    println!("part2: {}", count)
}
