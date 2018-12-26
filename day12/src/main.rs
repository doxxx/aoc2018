use shared::copy_into_array;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::ops::Index;
use std::ops::IndexMut;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let (initial_state, rules) = read_input()?;

    grow(&initial_state, &rules, 20);

    Ok(())
}

fn read_input() -> Result<(Vec<bool>, Vec<Rule>)> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines().map(str::trim);
    let initial_state = lines.next().map(parse_initial_state).unwrap();
    assert!(lines.next().unwrap().len() == 0);
    let rules = lines.map(parse_rule).collect();
    Ok((initial_state, rules))
}

fn parse_initial_state(s: &str) -> Vec<bool> {
    if !s.starts_with("initial state: ") {
        panic!();
    }

    parse_plant_states(&s[15..])
}

fn parse_plant_states(s: &str) -> Vec<bool> {
    s.chars().map(parse_plant_state).collect()
}

fn parse_plant_state(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("unexpected plant state character: {:?}", c),
    }
}

fn parse_rule(s: &str) -> Rule {
    let mut parts = s.split_whitespace();
    let expected_vec = parts.next().map(parse_plant_states).unwrap();
    assert_eq!(5, expected_vec.len());
    let mut expected: [bool; 5] = copy_into_array(&expected_vec);
    expected.copy_from_slice(&expected_vec);
    assert_eq!("=>", parts.next().unwrap());
    let output = parts
        .next()
        .map(|p| parse_plant_state(p.chars().next().unwrap()))
        .unwrap();
    Rule { expected, output }
}

struct Rule {
    expected: [bool; 5],
    output: bool,
}

impl Rule {
    fn matches(&self, pots: &[bool]) -> bool {
        for (i, &expected) in self.expected.iter().enumerate() {
            if pots[i] != expected {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pattern = String::from_iter(self.expected.iter().map(|&p| if p { '#' } else { '.' }));
        let output = if self.output { '#' } else { '.' };
        write!(f, "{} -> {}", pattern, output)
    }
}

fn grow(initial_state: &[bool], rules: &[Rule], gens: usize) {
    let mut pots = Pots::new(initial_state);

    // println!("0: {}", pots);

    for gen in 1..=gens {
        pots.grow(rules);
        // println!("{}: {}", gen, pots);
        // if gen % 1000 == 0 {
        //     println!("{}: {}", gen, pots);
        // }
    }

    let sum: isize = (pots.left_extent()..=pots.right_extent())
        .map(|i| if pots[i] { i } else { 0 })
        .sum();

    println!("result: {}", sum);
}

#[derive(Clone)]
struct Pots {
    pots: Vec<bool>,
    zero: isize,
}

impl Pots {
    fn new(initial_state: &[bool]) -> Pots {
        let mut pots = vec![false; initial_state.len() * 2];
        let zero = pots.len() / 2;
        pots[zero..zero + initial_state.len()].copy_from_slice(initial_state);
        Pots {
            pots,
            zero: zero as isize,
        }
    }

    fn grow(&mut self, rules: &[Rule]) {
        let mut new = self.clone();
        let mut leftmost = self.leftmost();
        let mut rightmost = self.rightmost();

        for i in self.left_extent() + 2..=self.right_extent() - 2 {
            new[i] = false;
            for rule in rules {
                if rule.matches(&self[(i - 2)..(i + 3)]) {
                    // println!("matched rule {} at {}", rule, i);
                    new[i] = rule.output;
                    break;
                }
            }
            if new[i] {
                if i < leftmost {
                    leftmost = i;
                }
                if i > rightmost {
                    rightmost = i;
                }
            }
        }

        if new.left_extent() - leftmost > -3 {
            new.extend_left(5);
        }
        if new.right_extent() - rightmost < 3 {
            new.extend_right(5);
        }

        *self = new;
    }

    fn extend_left(&mut self, n: usize) {
        // println!("before resize: {:?}", self.pots);
        let new_len = self.pots.len() + n;
        self.pots.resize(new_len, false);
        // println!("after resize:  {:?}", self.pots);
        for i in (n..new_len).rev() {
            self.pots[i] = self.pots[i - n];
        }
        // println!("after shift:   {:?}", self.pots);
        for i in 0..n {
            self.pots[i] = false;
        }
        // println!("after zero:    {:?}", self.pots);
        self.zero += n as isize;
    }

    fn extend_right(&mut self, n: usize) {
        let new_len = self.pots.len() + n;
        self.pots.resize(new_len, false);
    }

    fn left_extent(&self) -> isize {
        0 - self.zero
    }

    fn right_extent(&self) -> isize {
        (self.pots.len() as isize) - self.zero - 1
    }

    fn leftmost(&self) -> isize {
        for i in 0..self.pots.len() {
            if self.pots[i] {
                return (i as isize) - self.zero;
            }
        }

        panic!();
    }

    fn rightmost(&self) -> isize {
        for i in (0..self.pots.len()).rev() {
            if self.pots[i] {
                return (i as isize) - self.zero;
            }
        }

        panic!();
    }
}

impl std::fmt::Display for Pots {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = String::from_iter(self.pots.iter().map(|&p| if p { '#' } else { '.' }));
        write!(f, "{}", s)
    }
}

impl Index<isize> for Pots {
    type Output = bool;

    fn index(&self, index: isize) -> &bool {
        &self.pots[(self.zero + index) as usize]
    }
}

impl Index<std::ops::Range<isize>> for Pots {
    type Output = [bool];

    fn index(&self, index: std::ops::Range<isize>) -> &[bool] {
        let start = (self.zero + index.start) as usize;
        let end = (self.zero + index.end) as usize;
        &self.pots[start..end]
    }
}

impl IndexMut<isize> for Pots {
    fn index_mut(&mut self, index: isize) -> &mut bool {
        &mut self.pots[(self.zero + index) as usize]
    }
}
