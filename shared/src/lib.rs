use std::convert::AsMut;

#[derive(Clone)]
pub struct Grid<T> {
    pub size: usize,
    pub cells: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(size: usize, initial_value: T) -> Grid<T> {
        Grid {
            size,
            cells: vec![initial_value; size * size],
        }
    }

    pub fn new_with<F>(size: usize, cell_value_fn: F) -> Grid<T>
    where
        F: Fn(usize, usize) -> T,
    {
        let mut g = Grid {
            size,
            cells: Vec::with_capacity(size * size),
        };
        for y in 0..size {
            for x in 0..size {
                g.cells.push(cell_value_fn(x, y));
            }
        }
        g
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.cells[(y * self.size + x) as usize]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.cells[(y * self.size + x) as usize]
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: &self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T: Clone> Iterator for GridIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.grid.size {
            let (x, y, val) = (self.x, self.y, self.grid.get(self.x, self.y));
            self.x += 1;
            if self.x == self.grid.size {
                self.x = 0;
                self.y += 1;
            }
            Some((x, y, val))
        } else {
            None
        }
    }
}

impl<T: Clone> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &T {
        let (x, y) = coords;
        self.get(x, y)
    }
}

impl<T: Clone> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut T {
        let (x, y) = coords;
        self.get_mut(x, y)
    }
}

pub fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}
