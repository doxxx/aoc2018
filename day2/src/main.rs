use std::collections::HashMap;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let box_ids = read_input()?;

    part1(&box_ids);

    Ok(())
}

fn read_input() -> Result<Vec<String>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.lines().map(|s| s.to_owned()).collect())
}

fn part1(box_ids: &[String]) {
    let mut two: usize = 0;
    let mut three: usize = 0;

    for box_id in box_ids {
        if find_dupes(box_id, 2) {
            two += 1;
        }
        if find_dupes(box_id, 3) {
            three += 1;
        }
    }

    println!("part1: {}", two * three);
}

fn find_dupes(box_id: &str, target: usize) -> bool {
    let mut counts = HashMap::new();

    for letter in box_id.chars() {
        if let Some(count) = counts.get_mut(&letter) {
            (*count) += 1;
        } else {
            counts.insert(letter, 1);
        }
    }

    for (_, count) in counts {
        if count == target {
            return true;
        }
    }
    
    false
}
