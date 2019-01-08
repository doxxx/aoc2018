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

fn combined_map(map: &Map, units: &[Unit]) -> Map {
    let mut combined_map = map.clone();
    for unit in units.iter() {
        if unit.hp > 0 {
            combined_map[unit.pos] = unit.kind;
        }
    }
    combined_map
}

struct System {
    map: Map,
    units: Vec<Unit>,
}

#[derive(Debug)]
enum MoveResult {
    NoEnemies,
    Move(Pos),
    CannotMove,
    InRange,
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
            print!(
                "Unit {}@{:?} tries to move... ",
                self.units[i].kind, self.units[i].pos
            );

            match self.try_move(&self.units[i]) {
                MoveResult::NoEnemies => {
                    println!("no more enemies; ending combat.");
                    self.clear_dead();
                    self.print_summary(rounds);
                    return false;
                }
                MoveResult::Move(new_pos) => {
                    println!("moving to {:?}.", new_pos);
                    self.map[self.units[i].pos] = '.';
                    self.units[i].pos = new_pos;
                    self.map[self.units[i].pos] = self.units[i].kind;
                }
                MoveResult::CannotMove => {
                    println!("cannot move.");
                    continue;
                }
                MoveResult::InRange => {
                    println!("in range of enemy.");
                }
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

    fn try_move(&self, unit: &Unit) -> MoveResult {
        let enemies: Vec<&Unit> = self
            .units
            .iter()
            .filter(|u| u.kind != unit.kind && u.hp > 0)
            .collect();
        if enemies.is_empty() {
            return MoveResult::NoEnemies;
        }

        if enemies.iter().any(|u| unit.in_range(&u)) {
            return MoveResult::InRange;
        }

        let unit_positions: Vec<Pos> = self
            .units
            .iter()
            .filter(|u| u.hp > 0)
            .map(|u| u.pos)
            .collect();
        // println!("unit_positions={:?}", unit_positions);
        let open_squares: Vec<Pos> = enemies
            .iter()
            .flat_map(|u| self.map.open_squares_around(u.pos.0, u.pos.1))
            .filter(|p| !unit_positions.contains(p))
            .collect();
        // println!("open_squares={:?}", open_squares);

        if open_squares.is_empty() {
            return MoveResult::CannotMove;
        }

        let mut paths: Vec<Vec<Pos>> = open_squares
            .into_iter()
            .flat_map(|pos| {
                self.map
                    .shortest_paths(unit.pos.0, unit.pos.1, pos.0, pos.1)
            })
            .collect();

        if paths.is_empty() {
            return MoveResult::CannotMove;
        }

        paths.sort_by_key(|p| p.len());

        let fewest_steps = paths.first().unwrap().len();
        let mut shortest_paths: Vec<Vec<Pos>> = paths
            .into_iter()
            .filter(|p| p.len() == fewest_steps)
            .collect();
        shortest_paths.sort_by(|a, b| cmp_pos(a[0], b[0]));
        let shortest_path = shortest_paths.first().unwrap();
        MoveResult::Move(shortest_path.first().cloned().unwrap())
    }

    fn attack(&self, unit: &Unit) -> Option<(Pos, usize)> {
        let mut enemies: Vec<&Unit> = self
            .units
            .iter()
            .filter(|u| u.kind != unit.kind && u.hp > 0 && unit.in_range(&u))
            .collect();

        if enemies.is_empty() {
            return None;
        }

        enemies.sort_by_key(|u| u.hp);
        let fewest_hp = enemies.first().unwrap().hp;
        let mut enemies: Vec<&Unit> = enemies.into_iter().filter(|u| u.hp == fewest_hp).collect();
        enemies.sort_by(|a, b| cmp_pos(a.pos, b.pos));
        enemies.first().map(|t| (t.pos, unit.ap))
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
                // TODO: Use rpds?
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
