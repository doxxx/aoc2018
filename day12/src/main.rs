use shared::copy_into_array;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::ops::Index;
use std::ops::IndexMut;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let (initial_state, rules) = read_input()?;

    grow(&initial_state, &rules, 20);
    grow(&initial_state, &rules, 50000000000);

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
    let mut current = Pots::new(initial_state);

    // println!("0: {}", pots);

    for gen in 1..=gens {
        let new = current.grow(rules);
        if new.pots == current.pots {
            current = Pots {
                pots: new.pots,
                offset: (new.offset - current.offset) * (gens - gen + 1) as isize + current.offset,
            };
            break;
        } else {
            current = new;
        }
        // println!("{}: {}", gen, pots);
        if gen % 1000000 == 0 {
            println!("{}: {}", gen, current);
        }
    }

    let sum: isize = (current.left_extent()..=current.right_extent())
        .map(|i| if current[i] { i } else { 0 })
        .sum();

    println!("result: {}", sum);
}

#[derive(Clone)]
struct Pots {
    pots: Vec<bool>,
    offset: isize,
}

impl Pots {
    fn new(initial_state: &[bool]) -> Pots {
        let mut pots = vec![false; initial_state.len() + 10];
        pots[5..5 + initial_state.len()].copy_from_slice(initial_state);
        Pots { pots, offset: -5 }
    }

    fn grow(&mut self, rules: &[Rule]) -> Pots {
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

        if leftmost > new.left_extent() + 3 {
            let len = (rightmost - leftmost) as usize + 6;
            let leftmost_index = (leftmost - self.offset) as usize - 3;
            overlapping_copy(&mut new.pots, leftmost_index..(leftmost_index + len), 0);
            new.pots.truncate(len);
            new.offset += leftmost_index as isize;
        }

        if leftmost - new.left_extent() < 3 {
            new.extend_left(3);
        }
        if new.right_extent() - rightmost < 3 {
            new.extend_right(3);
        }

        new
    }

    fn extend_left(&mut self, n: usize) {
        let new_len = self.pots.len() + n;
        self.pots.resize(new_len, false);
        overlapping_copy(&mut self.pots, n..new_len, 0);
        for i in 0..n {
            self.pots[i] = false;
        }
        self.offset -= n as isize;
    }

    fn extend_right(&mut self, n: usize) {
        let new_len = self.pots.len() + n;
        self.pots.resize(new_len, false);
    }

    fn left_extent(&self) -> isize {
        self.offset
    }

    fn right_extent(&self) -> isize {
        self.offset + (self.pots.len() as isize) - 1
    }

    fn leftmost(&self) -> isize {
        for i in self.left_extent()..=self.right_extent() {
            if self[i] {
                return i;
            }
        }

        panic!();
    }

    fn rightmost(&self) -> isize {
        for i in (self.left_extent()..=self.right_extent()).rev() {
            if self[i] {
                return i;
            }
        }

        panic!();
    }
}

impl std::fmt::Display for Pots {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = String::from_iter(self.pots.iter().map(|&p| if p { '#' } else { '.' }));
        write!(f, "{:+} {}", self.offset, s)
    }
}

impl Index<isize> for Pots {
    type Output = bool;

    fn index(&self, index: isize) -> &bool {
        &self.pots[(index - self.offset) as usize]
    }
}

impl Index<std::ops::Range<isize>> for Pots {
    type Output = [bool];

    fn index(&self, index: std::ops::Range<isize>) -> &[bool] {
        let start = (index.start - self.offset) as usize;
        let end = (index.end - self.offset) as usize;
        &self.pots[start..end]
    }
}

impl IndexMut<isize> for Pots {
    fn index_mut(&mut self, index: isize) -> &mut bool {
        &mut self.pots[(index - self.offset) as usize]
    }
}

fn overlapping_copy<T, From>(s: &mut [T], from: From, to: usize)
where
    T: Copy,
    From: std::ops::RangeBounds<usize> + std::iter::Iterator,
{
    let from_start = match from.start_bound() {
        std::ops::Bound::Unbounded => 0,
        std::ops::Bound::Included(i) => *i,
        std::ops::Bound::Excluded(i) => *i + 1,
    };
    let from_end = match from.end_bound() {
        std::ops::Bound::Unbounded => s.len() - 1,
        std::ops::Bound::Included(i) => *i,
        std::ops::Bound::Excluded(i) => *i - 1,
    };

    if to < from_start {
        for i in from_start..=from_end {
            s[i - from_start + to] = s[i];
        }
    } else if to > from_start {
        for i in (from_start..=from_end).rev() {
            s[i - from_start + to] = s[i];
        }
    } else {
        panic!();
    }
}
