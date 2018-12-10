use std::iter::FromIterator;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::rc::Rc;
use std::cmp::Eq;
use std::hash::Hash;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let pairs = read_input()?;

    part1(&pairs);

    Ok(())
}

fn read_input() -> Result<Vec<(char, char)>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.trim_end().lines().map(parse_instruction).collect())
}

fn parse_instruction(s: &str) -> (char, char) {
    lazy_static! {
        static ref re: Regex =
            Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    }

    let caps = re.captures(s).unwrap();

    (caps[1].chars().next().unwrap(), caps[2].chars().next().unwrap())
}

fn part1(pairs: &[(char, char)]) {
    let steps = collect_steps(pairs);
    let rules = build_rules(pairs);
    let mut completed = Vec::new();

    while let Some(next) = find_next(&steps, &completed, &rules) {
        completed.push(next);
    }

    let result = String::from_iter(completed.into_iter());

    println!("part1: {}", result);
}

fn collect_steps(pairs: &[(char, char)]) -> Vec<char> {
    let mut steps: HashSet<char> = HashSet::new();

    for &(from, to) in pairs {
        steps.insert(from);
        steps.insert(to);
    }

    steps.into_iter().collect()
}

fn build_rules(pairs: &[(char, char)]) -> HashMap<char, Vec<char>> {
    let mut rules: HashMap<char, Vec<char>> = HashMap::new();

    for &(from, to) in pairs {
        rules.entry(to).or_insert_with(|| Vec::new()).push(from);
    }

    rules
}

fn find_next(steps: &[char], completed: &[char], rules: &HashMap<char, Vec<char>>) -> Option<char> {
    let mut available = Vec::new();

    for step in steps {
        if !completed.contains(step) {
            if let Some(rule) = rules.get(step) {
                if rule.iter().all(|prereq| completed.contains(prereq)) {
                    available.push(*step);
                }
            } else {
                available.push(*step);
            }
        }
    }

    available.sort();
    available.into_iter().next()
}
