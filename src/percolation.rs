use union_find::{UnionFind, PathCompressionWeightedQuickUnion};

pub trait Percolation {
    fn open(&mut self, row: usize, col: usize);

    fn is_open(&self, row: usize, col: usize) -> bool;

    fn is_full(&self, row: usize, col: usize) -> bool;

    fn percolates(&self) -> bool;
}

trait PercolationExt: Percolation {

    fn cell_index(&self, row: usize, col: usize) -> usize;

    fn fill_neighbors(&mut self, row: usize, col: usize);
}

pub struct BruteForcePercolation {
    openness: Vec<bool>,
    fullness: Vec<bool>,
    side_size: usize
}

impl BruteForcePercolation {

    pub fn new(side_size: usize) -> BruteForcePercolation {
        let length = side_size * side_size;
        let mut openness = Vec::with_capacity(length);
        let mut fullness = Vec::with_capacity(length);
        for _ in 0..length {
            openness.push(false);
            fullness.push(false);
        }
        BruteForcePercolation {
            openness: openness,
            fullness: fullness,
            side_size: side_size
        }
    }
}

impl PercolationExt for BruteForcePercolation {

    #[inline]
    fn cell_index(&self, row: usize, col: usize) -> usize {
        (row - 1) * self.side_size + col - 1
    }

    fn fill_neighbors(&mut self, row: usize, col: usize) {
        if row < self.side_size && self.is_open(row + 1, col) && !self.is_full(row + 1, col) {
            let index = self.cell_index(row + 1, col);
            self.fullness[index] = true;
            self.fill_neighbors(row + 1, col);
        }
        if col < self.side_size && self.is_open(row, col + 1) && !self.is_full(row, col + 1) {
            let index = self.cell_index(row, col + 1);
            self.fullness[index] = true;
            self.fill_neighbors(row, col + 1);
        }
        if col > 1 && self.is_open(row, col - 1) && !self.is_full(row, col - 1) {
            let index = self.cell_index(row, col - 1);
            self.fullness[index] = true;
            self.fill_neighbors(row, col - 1);
        }
        if row > 1 && self.is_open(row - 1, col) && !self.is_full(row - 1, col) {
            let index = self.cell_index(row - 1, col);
            self.fullness[index] = true;
            self.fill_neighbors(row - 1, col);
        }
    }
}

impl Percolation for BruteForcePercolation {
    fn open(&mut self, row: usize, col: usize) {
        if !self.is_open(row, col) {
            let index = self.cell_index(row, col);
            self.openness[index] = true;
            if row > 1 && self.is_full(row - 1, col)
                    || row < self.side_size && self.is_full(row + 1, col) {
                self.fullness[index] = true;
                self.fill_neighbors(row, col);
            }
            if col > 1 && self.is_full(row, col - 1)
                    || col < self.side_size && self.is_full(row, col + 1){
                self.fullness[index] = true;
                self.fill_neighbors(row, col);
            }
            if row == 1 {
                self.fullness[index] = true;
                self.fill_neighbors(row, col);
            }
        }
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        let index = self.cell_index(row, col);
        self.openness[index]
    }

    fn is_full(&self, row: usize, col: usize) -> bool {
        let index = self.cell_index(row, col);
        self.fullness[index]
    }

    fn percolates(&self) -> bool {
        for col in 1..self.side_size + 1 {
            if self.is_full(self.side_size, col) {
                return true;
            }
        }
        false
    }
}

pub struct UnionFindPercolation {
    union_find: PathCompressionWeightedQuickUnion,
    virtual_cell: usize,
    openness: Vec<bool>,
    fullness: Vec<bool>,
    side_size: usize
}

impl UnionFindPercolation {

    pub fn new(side_size: usize) -> UnionFindPercolation {
        let length = side_size * side_size;
        let mut openness = Vec::with_capacity(length);
        let mut fullness = Vec::with_capacity(length);
        for _ in 0..length {
            openness.push(false);
            fullness.push(false);
        }
        let virtual_cell = length;
        let mut union_find = PathCompressionWeightedQuickUnion::new(length + 1);
        for p in 0..side_size {
            union_find.union(p, virtual_cell);
        }
        UnionFindPercolation {
            union_find: union_find,
            virtual_cell: virtual_cell,
            openness: openness,
            fullness: fullness,
            side_size: side_size
        }
    }

    #[inline]
    fn is_open_by_index(&self, index: usize) -> bool {
        self.openness[index]
    }

    #[inline]
    fn is_full_by_index(&self, index: usize) -> bool {
        self.fullness[index]
    }

    fn connect_with_top(&mut self, index: usize) -> bool {
        if self.has_top_neighbor(index) {
            let upper_index = index - self.side_size;
            self.union_find.union(index, upper_index);
            self.is_open_by_index(upper_index)
        }
        else {
            false
        }
    }

    #[inline]
    fn has_top_neighbor(&self, index: usize) -> bool {
        index >= self.side_size
    }

    fn connect_with_left(&mut self, index: usize) -> bool {
        if self.has_left_neighbor(index) {
            let left_index = index - 1;
            self.union_find.union(index, left_index);
            self.is_open_by_index(left_index)
        }
        else {
            false
        }
    }

    #[inline]
    fn has_left_neighbor(&self, index: usize) -> bool {
        index % self.side_size > 0
    }

    fn connect_with_right(&mut self, index: usize) -> bool {
        if self.has_right_neighbor(index) {
            let right_index = index + 1;
            self.union_find.union(index, right_index);
            self.is_open_by_index(right_index)
        }
        else {
            false
        }
    }

    #[inline]
    fn has_right_neighbor(&self, index: usize) -> bool {
        index % self.side_size < self.side_size - 1
    }

    fn connect_with_bottom(&mut self, index: usize) -> bool {
        if self.has_bottom_neighbor(index) {
            let bottom_index = index + self.side_size;
            self.union_find.union(index, bottom_index);
            self.is_open_by_index(bottom_index)
        }
        else {
            false
        }
    }

    #[inline]
    fn has_bottom_neighbor(&self, index: usize) -> bool {
        index < self.side_size * (self.side_size - 1)
    }
}

impl Percolation for UnionFindPercolation {

    fn open(&mut self, row: usize, col: usize) {
        if !self.is_open(row, col) {
            let index = self.cell_index(row, col);
            self.openness[index] = true;
            let union = self.connect_with_top(index)
                | self.connect_with_left(index)
                | self.connect_with_right(index)
                | self.connect_with_bottom(index);
            if (union || row == 1) && self.union_find.connected(index, self.virtual_cell) {
                self.fullness[index] = true;
                self.fill_neighbors(row, col);
            }
        }
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        let index = self.cell_index(row, col);
        self.is_open_by_index(index)
    }

    fn is_full(&self, row: usize, col: usize) -> bool {
        let index = self.cell_index(row, col);
        self.is_full_by_index(index)
    }

    fn percolates(&self) -> bool {
        for col in 1..self.side_size + 1 {
            if self.is_full(self.side_size, col) {
                return true;
            }
        }
        false
    }
}

impl PercolationExt for UnionFindPercolation {

    #[inline]
    fn cell_index(&self, row: usize, col: usize) -> usize {
        (row - 1) * self.side_size + col - 1
    }

    fn fill_neighbors(&mut self, row: usize, col: usize) {
        if row < self.side_size && self.is_open(row + 1, col) && !self.is_full(row + 1, col) {
            let index = self.cell_index(row + 1, col);
            self.fullness[index] = true;
            self.fill_neighbors(row + 1, col);
        }
        if col < self.side_size && self.is_open(row, col + 1) && !self.is_full(row, col + 1) {
            let index = self.cell_index(row, col + 1);
            self.fullness[index] = true;
            self.fill_neighbors(row, col + 1);
        }
        if col > 1 && self.is_open(row, col - 1) && !self.is_full(row, col - 1) {
            let index = self.cell_index(row, col - 1);
            self.fullness[index] = true;
            self.fill_neighbors(row, col - 1);
        }
        if row > 1 && self.is_open(row - 1, col) && !self.is_full(row - 1, col) {
            let index = self.cell_index(row - 1, col);
            self.fullness[index] = true;
            self.fill_neighbors(row - 1, col);
        }
    }
}
