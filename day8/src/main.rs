use std::io::prelude::*;
use std::iter::FromIterator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let data = read_input()?;

    part1(&data);
    part2(&data);

    Ok(())
}

fn read_input() -> Result<Vec<usize>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect())
}

fn part1(data: &[usize]) {
    let tree = build_tree(&mut data.iter().map(|d| *d));

    let result = tree.sum_metadata();

    println!("part1: {}", result);
}

fn build_tree(data: &mut Iterator<Item = usize>) -> Node {
    let num_children = data.next().unwrap();
    let num_metadata = data.next().unwrap();
    let mut children = Vec::new();
    for _ in 0..num_children {
        let child = build_tree(data);
        children.push(child);
    }
    let mut metadata = Vec::new();
    for _ in 0..num_metadata {
        metadata.push(data.next().unwrap());
    }
    Node { children, metadata }
}

fn part2(data: &[usize]) {
    let tree = build_tree(&mut data.iter().map(|d| *d));
    let result = tree.value();
    println!("part2: {}", result);
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn print(&self, indent: usize) {
        println!(
            "{}Node: {:?}",
            String::from_iter(vec![' '; indent].into_iter()),
            self.metadata
        );
        for child in &self.children {
            child.print(indent + 2);
        }
    }

    fn sum_metadata(&self) -> usize {
        self.metadata.iter().map(|m| *m).sum::<usize>()
            + self
                .children
                .iter()
                .map(|n| n.sum_metadata())
                .sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().map(|m| *m).sum::<usize>()
        } else {
            self.metadata
                .iter()
                .map(|m| self.children.get(*m - 1).map_or(0, |n| n.value()))
                .sum::<usize>()
        }
    }
}
