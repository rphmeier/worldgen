use std::ops::{Index, IndexMut};

/// A rectangular grid of objects.
///
/// It can be indexed with an (x, y) or [x, y] pair. 
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    /// Create a grid of given width and height, with each entry set to the
    /// supplied value.
    pub fn with_value(width: usize, height: usize, val: T) -> Self {
        Grid {
            width: width,
            height: height,
            data: (0..(width * height)).map(|_| val.clone()).collect(),
        }
    }
}

impl<T: Default> Grid<T> {
    /// Create a grid of given width and height, with each entry set to the
    /// type's default.
    pub fn with_default(width: usize, height: usize) -> Self {
        Grid {
            width: width,
            height: height,
            data: (0..(width * height)).map(|_| Default::default()).collect(),
        }
    }
}

impl<T> Grid<T> {
    /// Returns the width of this grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of this grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Perform checked indexing into the grid.
    /// This will return `None` if x or y is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(unsafe {
            self.get_unchecked(x, y)
        })
    }

    /// Perform checked mutable indexing into the grid.
    /// This will return `None` if x or y is out of bounds.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(unsafe {
            self.get_unchecked_mut(x, y)
        })
    }

    /// Perform unchecked indexing into the grid.
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.data.get_unchecked(y * self.height + x)
    }

    /// Perform unchecked mutable indexing into the grid.
    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.data.get_unchecked_mut(y * self.height + x)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        let (x, y) = index;

        assert!(x < self.width,
            "Index out of bounds: x is {} but the width is {}", x, self.width);
        assert!(y < self.height,
            "Index out of bounds: y is {} but the height is {}", y, self.height);

        unsafe { self.get_unchecked(x, y) }
    }
}

impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &T {
        &self[(index[0], index[1])]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        let (x, y) = index;

        assert!(x < self.width,
            "Index out of bounds: x is {} but the width is {}", x, self.width);
        assert!(y < self.height,
            "Index out of bounds: y is {} but the height is {}", y, self.height);

        unsafe { self.get_unchecked_mut(x, y) }
    }
}

impl<T> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
        &mut self[(index[0], index[1])]
    }
}