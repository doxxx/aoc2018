use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::iter::FromIterator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let pairs = read_input()?;

    part1(&pairs);
    part2(&pairs, 60, 5);

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

    (
        caps[1].chars().next().unwrap(),
        caps[2].chars().next().unwrap(),
    )
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

fn part2(pairs: &[(char, char)], base_time_per_step: i32, num_workers: usize) {
    let mut steps = collect_steps(pairs);
    steps.sort();

    let rules = build_rules(pairs);
    let mut completed = Vec::new();
    let mut workers = vec![Worker(None); num_workers];
    let mut time = 0;

    loop {
        for worker in workers.iter_mut() {
            if worker.is_idle() {
                if let Some(next) = find_next(&steps, &completed, &rules) {
                    steps.remove(steps.binary_search(&next).unwrap());
                    worker.assign(next, time_for_step(base_time_per_step, next));
                }
            }
        }

        print_status(time, &workers, &completed);

        for worker in workers.iter_mut() {
            if let Some(step) = worker.work() {
                completed.push(step);
            }
        }

        time += 1;

        if steps.is_empty() && workers.iter().all(|w| w.is_idle()) {
            print_status(time, &workers, &completed);
            break;
        }
    }

    println!("part2: {}", time);
}

fn time_for_step(base: i32, step: char) -> i32 {
    base + (step as i32) - ('A' as i32) + 1
}

fn print_status(time: i32, workers: &[Worker], completed: &[char]) {
    print!("{}\t", time);
    for worker in workers {
        print!("{:?}\t", worker.0.unwrap_or(('.', 0)));
    }
    println!("{}", String::from_iter(completed.iter()));
}

#[derive(Clone)]
struct Worker(Option<(char, i32)>);

impl Worker {
    fn is_idle(&self) -> bool {
        self.0.is_none()
    }

    fn assign(&mut self, step: char, remaining: i32) {
        assert!(self.is_idle());
        self.0 = Some((step, remaining));
    }

    fn work(&mut self) -> Option<char> {
        if let Some((step, remaining)) = self.0 {
            if remaining > 1 {
                self.0.replace((step, remaining - 1));
                None
            } else {
                self.0.take().map(|(step, _)| step)
            }
        } else {
            None
        }
    }
}
