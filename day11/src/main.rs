use rayon::prelude::*;
use shared::Grid;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let serial = read_input()?;

    part1(serial);
    part2(serial);

    Ok(())
}

fn read_input() -> Result<i64> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.trim().parse().unwrap())
}

fn part1(serial: i64) {
    let g: FuelCellGrid = Grid::new_with(300, |x, y| cell_power_level(x + 1, y + 1, serial));
    let (_, x, y) = search_max_square_power(&g, 3);

    println!("part1: {},{}", x + 1, y + 1);
}

type FuelCellGrid = Grid<i64>;

fn cell_power_level(x: usize, y: usize, serial: i64) -> i64 {
    let rack_id = (x as i64) + 10;
    let mut result = rack_id * (y as i64);
    result += serial;
    result *= rack_id;
    result = (result / 100) % 10;
    result -= 5;
    result
}

fn search_max_square_power(g: &FuelCellGrid, square_size: usize) -> (i64, usize, usize) {
    let mut max_power_level = (0, 0, 0);

    for y in 0..(g.size - square_size) {
        for x in 0..(g.size - square_size) {
            let power_level = square_power_level(g, x, y, square_size);
            if power_level > max_power_level.0 {
                max_power_level = (power_level, x, y);
            }
        }
    }

    max_power_level
}

fn square_power_level(g: &Grid<i64>, x: usize, y: usize, size: usize) -> i64 {
    let mut result = 0;
    for j in y..(y + size) {
        for i in x..(x + size) {
            result += g[(i, j)];
        }
    }
    result
}

fn part2(serial: i64) {
    let g: FuelCellGrid = Grid::new_with(300, |x, y| cell_power_level(x + 1, y + 1, serial));

    let square_sizes: Vec<usize> = (1..=300).collect();

    let ((_, x, y), square_size) = square_sizes
        .par_iter()
        .map(|&square_size| (search_max_square_power(&g, square_size), square_size))
        .max_by_key(|((power, _, _), _)| *power)
        .unwrap();

    println!(
        "part1: {},{},{}",
        x + 1,
        y + 1,
        square_size
    );
}
