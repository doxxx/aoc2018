use shared::Grid;
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let (map, units) = read_input()?;

    run(&map, units.clone());

    Ok(())
}

fn read_input() -> std::io::Result<(Map, Vec<Unit>)> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let w = lines.first().unwrap().len();
    let h = lines.len();

    let mut map = Grid::new_with(w.max(h), |x, y| lines[y][x]);
    let mut units = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let c = map.get_mut(x, y);
            match *c {
                'G' | 'E' => {
                    units.push(Unit {
                        kind: *c,
                        x,
                        y,
                        hp: 200,
                        ap: 3,
                    });
                    *c = '.'
                }
                '#' => {}
                '.' => {}
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid map char: {}", c),
                    ));
                }
            }
        }
    }

    Ok((map, units))
}

fn run(map: &Map, mut units: Vec<Unit>) {
    for i in 0..3 {
        println!("Step {}:\n{}", i, combined_map(map, &units));

        // let unit_moves: Vec<Option<Pos>> = units
        //     .iter()
        //     .map(|u| u.move_to_nearest_target(map, &units))
        //     .collect();

        // //units.iter().zip(unit_moves.iter()).collect::<Vec<_>>()

        // units.iter_mut().zip(unit_moves).for_each(|(u, m)| {
        //     if let Some(m) = m {
        //         u.apply_move(m)
        //     }
        // });
    }

    println!("Final:\n{}", combined_map(map, &units));
}

fn combined_map(map: &Map, units: &[Unit]) -> Map {
    let mut combined_map = map.clone();
    for unit in units.iter() {
        combined_map[(unit.x, unit.y)] = unit.kind;
    }
    combined_map
}

type Map = Grid<char>;
type Pos = (usize, usize);

fn cmp_pos(a: Pos, b: Pos) -> Ordering {
    let (ax, ay) = a;
    let (bx, by) = b;
    if ay < by {
        Ordering::Less
    } else if ay == by {
        if ax < bx {
            Ordering::Less
        } else if ax > bx {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    } else {
        Ordering::Greater
    }
}

trait MapOps {
    fn open_squares_around(&self, x: usize, y: usize) -> Vec<Pos>;
    fn shortest_paths(&self, sx: usize, sy: usize, dx: usize, dy: usize) -> Vec<Vec<Pos>>;
}

impl MapOps for Map {
    fn open_squares_around(&self, x: usize, y: usize) -> Vec<Pos> {
        let mut open = Vec::new();
        if x > 0 && *self.get(x - 1, y) == '.' {
            open.push((x - 1, y));
        }
        if x < self.size - 1 && *self.get(x + 1, y) == '.' {
            open.push((x + 1, y));
        }
        if y > 0 && *self.get(x, y - 1) == '.' {
            open.push((x, y - 1));
        }
        if y < self.size - 1 && *self.get(x, y + 1) == '.' {
            open.push((x, y + 1));
        }
        open
    }

    fn shortest_paths(&self, sx: usize, sy: usize, dx: usize, dy: usize) -> Vec<Vec<Pos>> {
        let mut explorer = Explorer {
            map: self,
            dx,
            dy,
            paths: Vec::new(),
        };

        // try rpds crate if Vec doesn't perform well
        explorer.explore(sx, sy, Vec::new());

        explorer.paths
    }
}

struct Explorer<'a> {
    map: &'a Map,
    dx: usize,
    dy: usize,
    paths: Vec<Vec<Pos>>,
}

impl<'a> Explorer<'a> {
    fn explore(&mut self, sx: usize, sy: usize, path: Vec<Pos>) {
        if sx == self.dx && sy == self.dy {
            self.paths.push(path);
            return;
        }

        if self.dy < sy {
            self.step(sx, sy - 1, path.clone());
        }
        if self.dx > sx {
            self.step(sx + 1, sy, path.clone());
        }
        if self.dx < sx {
            self.step(sx - 1, sy, path.clone());
        }
        if self.dy > sy {
            self.step(sx, sy + 1, path.clone());
        }
    }

    fn step(&mut self, nx: usize, ny: usize, mut path: Vec<Pos>) {
        if *self.map.get(nx, ny) == '.' {
            path.push((nx, ny));
            self.explore(nx, ny, path);
        }
        // else deadend
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Unit {
    kind: char,
    x: usize,
    y: usize,
    hp: usize,
    ap: usize,
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else if self.y == other.y {
            if self.x < other.x {
                Ordering::Less
            } else if self.x > other.x {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Unit {
    fn in_range(&self, other: &Unit) -> bool {
        (self.x == other.x && (self.y - 1 == other.y || self.y + 1 == other.y))
            || (self.y == other.y && (self.x - 1 == other.x || self.x + 1 == other.x))
    }

    fn move_to_nearest_target(&mut self, map: &Map, units: &[Unit]) -> bool {
        if units.iter().any(|u| self.in_range(u)) {
            return false;
        }

        let unit_positions: Vec<Pos> = units.iter().map(|u| (u.x, u.y)).collect();
        let targets: Vec<&Unit> = units.iter().filter(|u| u.kind != self.kind).collect();
        let open_squares: Vec<Pos> = targets
            .iter()
            .flat_map(|u| map.open_squares_around(u.x, u.y))
            .filter(|p| !unit_positions.contains(p))
            .collect();

        if open_squares.is_empty() {
            return false;
        }

        let mut paths: Vec<Vec<Pos>> = open_squares
            .into_iter()
            .flat_map(|pos| map.shortest_paths(self.x, self.y, pos.0, pos.1))
            .collect();

        if paths.is_empty() {
            return false;
        }

        paths.sort_by_key(|p| p.len());
        let fewest_steps = paths.first().unwrap().len();

        let mut shortest_paths: Vec<Vec<Pos>> = paths
            .into_iter()
            .filter(|p| p.len() == fewest_steps)
            .collect();
        shortest_paths.sort_by(|a, b| cmp_pos(a[0], b[0]));
        let shortest_path = shortest_paths.first().unwrap();
        let (nx, ny) = shortest_path.first().cloned().unwrap();

        self.x = nx;
        self.y = ny;

        true
    }

    fn attack(&self, units: &mut [Unit]) -> Option<(Pos, usize)> {
        let mut in_range: Vec<&mut Unit> = units.iter_mut().filter(|u| self.in_range(u)).collect();
        if in_range.is_empty() {
            return None;
        }

        in_range.sort_by_key(|u| u.hp);
        let fewest_hp = in_range.first().unwrap().hp;

        let mut targets: Vec<&mut Unit> =
            in_range.into_iter().filter(|u| u.hp == fewest_hp).collect();
        targets.sort_by(|a, b| cmp_pos((a.x, a.y), (b.x, b.y)));
        targets[0].hp -= self.ap;

        Some(((target.x, target.y), self.ap))
    }
}
