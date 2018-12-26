use std::convert::AsMut;

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
    where F: Fn(usize,usize) -> T
    {
        let mut g = Grid {
            size,
            cells: Vec::with_capacity(size*size),
        };
        for y in 0..size {
            for x in 0..size {
                g.cells.push(cell_value_fn(x, y));
            }
        }
        g
    }
}

impl<T: Clone> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &T {
        let (x, y) = coords;
        &self.cells[(y * self.size + x) as usize]
    }
}

impl<T: Clone> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut T {
        let (x, y) = coords;
        &mut self.cells[(y * self.size + x) as usize]
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
