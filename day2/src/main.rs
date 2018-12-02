use std::collections::HashMap;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let box_ids = read_input()?;

    part1(&box_ids);
    part2(&box_ids);

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

fn part2(box_ids: &[String]) {
    for current in box_ids {
        for search in box_ids {
            let (common, diff) = find_common(current, search);
            if diff == 1 {
                println!("part2: {}", common);
                return;
            }
        }
    }
}

fn find_common(a: &str, b: &str) -> (String, usize) {
    let mut common = String::new();
    let mut diffs = 0;

    for (a_letter, b_letter) in a.chars().zip(b.chars()) {
        if a_letter == b_letter {
            common.push(a_letter);
        } else {
            diffs += 1;
        }
    }

    (common, diffs)
}
