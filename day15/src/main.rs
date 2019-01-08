use shared::Grid;
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut system = read_input()?;

    system.run();

    Ok(())
}

fn read_input() -> std::io::Result<System> {
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
                        pos: (x, y),
                        hp: 200,
                        ap: 3,
                    });
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

    Ok(System::new(map, units))
}

struct System {
    map: Map,
    units: Vec<Unit>,
}

#[derive(Debug)]
enum Enemies<'a> {
    None,
    InRange,
    OutOfRange(Vec<&'a Unit>),
}

impl System {
    fn new(map: Map, units: Vec<Unit>) -> System {
        System { map, units }
    }

    fn run(&mut self) {
        self.units.sort_by(|a, b| cmp_pos(a.pos, b.pos));
        println!("Initial:\n{}", self.map);
        self.units
            .iter()
            .for_each(|u| println!("{}@{:?}: {}", u.kind, u.pos, u.hp));
        println!();

        let mut rounds = 1;

        loop {
            println!("Round {}:", rounds);

            if !self.eval_round(rounds) {
                break;
            }

            rounds += 1;
        }

        let completed_rounds = rounds - 1;
        let outcome = self.units.iter().map(|u| u.hp).sum::<usize>() * completed_rounds;

        println!("Completed rounds: {}", completed_rounds);
        println!("Battle outcome: {}", outcome);
    }

    fn eval_round(&mut self, rounds: usize) -> bool {
        self.units.sort_by(|a, b| cmp_pos(a.pos, b.pos));

        for i in 0..self.units.len() {
            match self.check_enemies(&self.units[i]) {
                Enemies::None => {
                    println!(
                        "Unit {}@{:?} finds no more enemies; ending combat.",
                        self.units[i].kind, self.units[i].pos
                    );
                    self.clear_dead();
                    self.print_summary(rounds);
                    return false;
                }
                Enemies::InRange => {
                    println!(
                        "Unit {}@{:?} already in-range of enemies.",
                        self.units[i].kind, self.units[i].pos
                    );
                }
                Enemies::OutOfRange(enemies) => match self.try_move(&self.units[i], enemies) {
                    Some(new_pos) => {
                        println!(
                            "Unit {}@{:?} moving to {:?}.",
                            self.units[i].kind, self.units[i].pos, new_pos
                        );
                        self.map[self.units[i].pos] = '.';
                        self.units[i].pos = new_pos;
                        self.map[self.units[i].pos] = self.units[i].kind;
                    }
                    None => {
                        println!(
                            "Unit {}@{:?} cannot move.",
                            self.units[i].kind, self.units[i].pos
                        );
                        continue;
                    }
                },
            }

            if let Some((target, ap)) = self.attack(&self.units[i]) {
                if let Some(mut u) = self.units.iter_mut().find(|u| u.pos == target) {
                    print!("Attacking {}@{:?}... ", u.kind, u.pos);
                    if u.hp < ap {
                        println!("dead!");
                        u.hp = 0;
                        self.map[u.pos] = '.';
                    } else {
                        u.hp -= ap;
                        println!("{} HP lost! {} HP remaining.", ap, u.hp);
                    }
                }
            }
        }

        self.clear_dead();
        self.print_summary(rounds);

        true
    }

    fn clear_dead(&mut self) {
        self.units = self.units.iter().filter(|u| u.hp > 0).cloned().collect();
    }

    fn print_summary(&self, rounds: usize) {
        println!();
        println!("After round {}:\n{}", rounds, self.map);
        self.units
            .iter()
            .for_each(|u| println!("{}@{:?}: {}", u.kind, u.pos, u.hp));
        println!();
    }

    fn check_enemies<'a>(&'a self, unit: &Unit) -> Enemies<'a> {
        let enemies: Vec<&Unit> = self
            .units
            .iter()
            .filter(|e| e.kind != unit.kind && e.hp > 0)
            .collect();
        if enemies.is_empty() {
            return Enemies::None;
        }

        if enemies.iter().any(|e| unit.in_range(&e)) {
            return Enemies::InRange;
        }

        Enemies::OutOfRange(enemies)
    }

    fn try_move(&self, unit: &Unit, enemies: Vec<&Unit>) -> Option<Pos> {
        let open_squares: Vec<Pos> = enemies
            .iter()
            .flat_map(|e| self.map.open_squares_around(e.pos.0, e.pos.1))
            .collect();

        if open_squares.is_empty() {
            return None;
        }

        let mut shortest_paths: Vec<Vec<Pos>> = open_squares
            .into_iter()
            .flat_map(|dest| {
                self.map
                    .shortest_paths(unit.pos.0, unit.pos.1, dest.0, dest.1)
            })
            .collect();

        if shortest_paths.is_empty() {
            return None;
        }

        shortest_paths.sort_by(|a, b| cmp_pos(a[0], b[0]));
        let shortest_path = shortest_paths.first().unwrap();
        shortest_path.first().cloned()
    }

    fn attack(&self, unit: &Unit) -> Option<(Pos, usize)> {
        let mut enemies: Vec<&Unit> = self
            .units
            .iter()
            .filter(|&e| e.kind != unit.kind && e.hp > 0 && unit.in_range(e))
            .collect();

        if enemies.is_empty() {
            return None;
        }

        enemies.sort_by_key(|&e| e.hp);
        let fewest_hp = enemies.first().unwrap().hp;
        let mut enemies: Vec<&Unit> = enemies.into_iter().filter(|&e| e.hp == fewest_hp).collect();
        enemies.sort_by(|&a, &b| cmp_pos(a.pos, b.pos));
        enemies.first().map(|&e| (e.pos, unit.ap))
    }
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
            if self
                .paths
                .first()
                .map(|p| p.len() > path.len())
                .unwrap_or(true)
            {
                self.paths.clear();
            }
            self.paths.push(path);
            return;
        }

        if sy > 0 {
            self.step(sx, sy - 1, &path);
        }
        if sx > 0 {
            self.step(sx - 1, sy, &path);
        }
        if sx < self.map.size - 1 {
            self.step(sx + 1, sy, &path);
        }
        if sy < self.map.size - 1 {
            self.step(sx, sy + 1, &path);
        }
    }

    fn step(&mut self, nx: usize, ny: usize, path: &[Pos]) {
        if *self.map.get(nx, ny) == '.' {
            if !path.contains(&(nx, ny)) {
                let mut path = Vec::from(path);
                path.push((nx, ny));
                self.explore(nx, ny, path);
            }
        }

        // deadend
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Unit {
    kind: char,
    pos: Pos,
    hp: usize,
    ap: usize,
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_pos(self.pos, other.pos)
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Unit {
    fn in_range(&self, other: &Unit) -> bool {
        let (sx, sy) = self.pos;
        let (ox, oy) = other.pos;
        (sx == ox && (sy - 1 == oy || sy + 1 == oy)) || (sy == oy && (sx - 1 == ox || sx + 1 == ox))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn compare_positions() {
        let top = (3, 3);
        let left = (2, 4);
        let right = (4, 4);
        let bottom = (3, 5);

        assert_eq!(Ordering::Less, cmp_pos(top, left));
        assert_eq!(Ordering::Less, cmp_pos(left, right));
        assert_eq!(Ordering::Less, cmp_pos(right, bottom));
    }
}
