use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use fnv::FnvHasher;

use union_find::{UnionFind, PathCompressionWeightedQuickUnion};
use generator::{Generator, DigitBaseGenerator};
use self::ext::{PercolationBase, Ext};

mod ext {
    use self::State::{Close, Open, Full};

    pub trait Ext {

        fn cell_index(&self, row: usize, col: usize) -> usize;

        fn fill_neighbors(&mut self, index: usize);

        fn is_open_by_index(&self, index: usize) -> bool;

        fn is_full_by_index(&self, index: usize) -> bool;

        fn has_top_neighbor(&self, index: usize) -> bool;

        fn has_left_neighbor(&self, index: usize) -> bool;

        fn has_right_neighbor(&self, index: usize) -> bool;

        fn has_bottom_neighbor(&self, index: usize) -> bool;

        fn make_open(&mut self, index: usize);

        fn make_full(&mut self, index: usize);
    }

    #[derive(Ord, Eq, PartialEq, PartialOrd)]
    enum State {
        Close,
        Open,
        Full
    }

    pub struct PercolationBase {
        states: Vec<State>,
        side_size: usize
    }

    impl PercolationBase {

        pub fn new(side_size: usize) -> PercolationBase {
            let length = side_size * side_size;
            let mut states = Vec::with_capacity(length);
            for _ in 0..length {
                states.push(Close);
            }
            PercolationBase {
                states: states,
                side_size: side_size
            }
        }

        pub fn side_size(&self) -> usize {
            self.side_size
        }
    }

    impl Ext for PercolationBase {

        #[inline]
        fn is_open_by_index(&self, index: usize) -> bool {
            self.states[index] > Close
        }

        #[inline]
        fn is_full_by_index(&self, index: usize) -> bool {
            self.states[index] > Open
        }

        #[inline]
        fn has_top_neighbor(&self, index: usize) -> bool {
            index >= self.side_size
        }

        #[inline]
        fn has_left_neighbor(&self, index: usize) -> bool {
            index % self.side_size > 0
        }

        #[inline]
        fn has_right_neighbor(&self, index: usize) -> bool {
            index % self.side_size < self.side_size - 1
        }

        #[inline]
        fn has_bottom_neighbor(&self, index: usize) -> bool {
            index < self.side_size * (self.side_size - 1)
        }

        #[inline]
        fn cell_index(&self, row: usize, col: usize) -> usize {
            (row - 1) * self.side_size + col - 1
        }

        fn make_open(&mut self, index: usize) {
            self.states[index] = Open;
        }

        fn make_full(&mut self, index: usize) {
            self.states[index] = Full;
        }

        fn fill_neighbors(&mut self, index: usize) {
            if self.has_top_neighbor(index) {
                let upper_index = index - self.side_size;
                if self.is_open_by_index(index) && !self.is_full_by_index(upper_index) {
                    self.make_full(upper_index);
                    self.fill_neighbors(upper_index);
                }
            }
            if self.has_right_neighbor(index) {
                let right_index = index + 1;
                if self.is_open_by_index(right_index) && !self.is_full_by_index(right_index) {
                    self.make_full(right_index);
                    self.fill_neighbors(right_index);
                }
            }
            if self.has_left_neighbor(index) {
                let left_index = index - 1;
                if self.is_open_by_index(left_index) && !self.is_full_by_index(left_index) {
                    self.make_full(left_index);
                    self.fill_neighbors(left_index);
                }
            }
            if self.has_bottom_neighbor(index) {
                let bottom_index = index + self.side_size;
                if self.is_open_by_index(bottom_index) && !self.is_full_by_index(bottom_index) {
                    self.make_full(bottom_index);
                    self.fill_neighbors(bottom_index);
                }
            }
        }
    }

}

pub trait Percolation {

    fn open(&mut self, row: usize, col: usize);

    fn is_open(&self, row: usize, col: usize) -> bool;

    fn is_full(&self, row: usize, col: usize) -> bool;

    fn percolates(&self) -> bool;
}

pub struct BruteForcePercolation {
    base: PercolationBase
}

impl BruteForcePercolation {

    pub fn new(side_size: usize) -> BruteForcePercolation {
        BruteForcePercolation {
            base: PercolationBase::new(side_size)
        }
    }
}

impl Percolation for BruteForcePercolation {

    fn open(&mut self, row: usize, col: usize) {
        if !self.is_open(row, col) {
            let index = self.base.cell_index(row, col);
            self.base.make_open(index);
            if self.base.has_top_neighbor(index) && self.is_full(row - 1, col)
                    || self.base.has_bottom_neighbor(index) && self.is_full(row + 1, col) {
                self.base.make_full(index);
                self.base.fill_neighbors(index);
            }
            if self.base.has_left_neighbor(index) && self.is_full(row, col - 1)
                    || self.base.has_right_neighbor(index) && self.is_full(row, col + 1){
                self.base.make_full(index);
                self.base.fill_neighbors(index);
            }
            if row == 1 {
                self.base.make_full(index);
                self.base.fill_neighbors(index);
            }
        }
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        let index = self.base.cell_index(row, col);
        self.base.is_open_by_index(index)
    }

    fn is_full(&self, row: usize, col: usize) -> bool {
        let index = self.base.cell_index(row, col);
        self.base.is_full_by_index(index)
    }

    fn percolates(&self) -> bool {
        for col in (1..).take(self.base.side_size()) {
            if self.is_full(self.base.side_size(), col) {
                return true;
            }
        }
        false
    }
}

pub struct UnionFindPercolation {
    union_find: PathCompressionWeightedQuickUnion,
    virtual_cell: usize,
    base: PercolationBase
}

impl UnionFindPercolation {

    pub fn new(side_size: usize) -> UnionFindPercolation {
        let length = side_size * side_size;
        let virtual_cell = length;
        let mut union_find = PathCompressionWeightedQuickUnion::new(length + 1);
        for p in 0..side_size {
            union_find.union(p, virtual_cell);
        }
        UnionFindPercolation {
            union_find: union_find,
            virtual_cell: virtual_cell,
            base: PercolationBase::new(side_size)
        }
    }

    fn connect_with_top(&mut self, index: usize) -> bool {
        if self.base.has_top_neighbor(index) {
            let upper_index = index - self.base.side_size();
            self.union_find.union(index, upper_index);
            self.base.is_open_by_index(upper_index)
        }
        else {
            false
        }
    }

    fn connect_with_left(&mut self, index: usize) -> bool {
        if self.base.has_left_neighbor(index) {
            let left_index = index - 1;
            self.union_find.union(index, left_index);
            self.base.is_open_by_index(left_index)
        }
        else {
            false
        }
    }

    fn connect_with_right(&mut self, index: usize) -> bool {
        if self.base.has_right_neighbor(index) {
            let right_index = index + 1;
            self.union_find.union(index, right_index);
            self.base.is_open_by_index(right_index)
        }
        else {
            false
        }
    }

    fn connect_with_bottom(&mut self, index: usize) -> bool {
        if self.base.has_bottom_neighbor(index) {
            let bottom_index = index + self.base.side_size();
            self.union_find.union(index, bottom_index);
            self.base.is_open_by_index(bottom_index)
        }
        else {
            false
        }
    }
}

impl Percolation for UnionFindPercolation {

    fn open(&mut self, row: usize, col: usize) {
        if !self.is_open(row, col) {
            let index = self.base.cell_index(row, col);
            self.base.make_open(index);
            let union = self.connect_with_top(index)
                | self.connect_with_left(index)
                | self.connect_with_right(index)
                | self.connect_with_bottom(index);
            if (union || row == 1) && self.union_find.connected(index, self.virtual_cell) {
                self.base.make_full(index);
                self.base.fill_neighbors(index);
            }
        }
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        let index = self.base.cell_index(row, col);
        self.base.is_open_by_index(index)
    }

    fn is_full(&self, row: usize, col: usize) -> bool {
        let index = self.base.cell_index(row, col);
        self.base.is_full_by_index(index)
    }

    fn percolates(&self) -> bool {
        for col in (1..).take(self.base.side_size()) {
            if self.is_full(self.base.side_size(), col) {
                return true;
            }
        }
        false
    }
}

fn round_up_to_next_highest_power_of_two(mut v: usize) -> usize {
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v += 1;
    v
}

type FasterHasher = BuildHasherDefault<FnvHasher>;

pub struct HackUnionFindPercolation {
    union_find: PathCompressionWeightedQuickUnion,
    virtual_cell: usize,
    openness: HashMap<usize, bool, FasterHasher>,
    fullness: HashMap<usize, bool, FasterHasher>,
    side_size: usize,
    lg_2: usize,
    mask: usize
}

impl HackUnionFindPercolation {

    pub fn new(side_size: usize) -> HackUnionFindPercolation {
        let length = side_size * side_size;
        let mut openness = HashMap::with_capacity_and_hasher(length, FasterHasher::default());
        let mut fullness = HashMap::with_capacity_and_hasher(length, FasterHasher::default());
        let power_of_two = round_up_to_next_highest_power_of_two(side_size);
        let mut generator = DigitBaseGenerator::new(power_of_two, side_size);
        for _ in 0..length {
            let key = generator.next();
            openness.insert(key, false);
            fullness.insert(key, false);
        }
        let virtual_cell = generator.next();
        let lg_2 = (power_of_two as f64).log2() as usize;
        let mut generator = DigitBaseGenerator::new(power_of_two, side_size);
        let mut union_find = PathCompressionWeightedQuickUnion::with_generator(length + 1, &mut generator);
        for p in 0..side_size {
            union_find.union(p, virtual_cell);
        }
        HackUnionFindPercolation {
            union_find: union_find,
            virtual_cell: virtual_cell,
            openness: openness,
            fullness: fullness,
            side_size: side_size,
            lg_2: lg_2,
            mask: power_of_two - 1
        }
    }

    fn connect_with_top(&mut self, index: usize) -> bool {
        if self.has_top_neighbor(index) {
            let diff = index & self.mask;
            let upper_index = (((index >> self.lg_2) - 1) << self.lg_2) | diff;
            self.union_find.union(index, upper_index);
            self.is_open_by_index(upper_index)
        }
        else {
            false
        }
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

    fn connect_with_bottom(&mut self, index: usize) -> bool {
        if self.has_bottom_neighbor(index) {
            let diff = index & self.mask;
            let bottom_index = (((index >> self.lg_2) + 1) << self.lg_2) | diff;
            self.union_find.union(index, bottom_index);
            self.is_open_by_index(bottom_index)
        }
        else {
            false
        }
    }

    #[inline]
    fn cell_index(&self, row: usize, col: usize) -> usize {
        ((row - 1) << self.lg_2) + col - 1
    }

    #[inline]
    fn is_open_by_index(&self, index: usize) -> bool {
        self.openness[&index]
    }

    #[inline]
    fn is_full_by_index(&self, index: usize) -> bool {
        self.fullness[&index]
    }

    #[inline]
    fn has_top_neighbor(&self, index: usize) -> bool {
        index >= self.side_size
    }

    #[inline]
    fn has_left_neighbor(&self, index: usize) -> bool {
        index & self.mask > 0
    }

    #[inline]
    fn has_right_neighbor(&self, index: usize) -> bool {
        index & self.mask < self.side_size - 1
    }

    #[inline]
    fn has_bottom_neighbor(&self, index: usize) -> bool {
        index >> self.lg_2 != self.side_size - 1
    }

    fn fill_neighbors(&mut self, index: usize) {
        if self.has_top_neighbor(index) {
            let diff = index & self.mask;
            let upper_index = (((index >> self.lg_2) - 1) << self.lg_2) | diff;
            if self.is_open_by_index(index) && !self.is_full_by_index(upper_index) {
                self.fullness.insert(upper_index, true);
                self.fill_neighbors(upper_index);
            }
        }
        if self.has_right_neighbor(index) {
            let right_index = index + 1;
            if self.is_open_by_index(right_index) && !self.is_full_by_index(right_index) {
                self.fullness.insert(right_index, true);
                self.fill_neighbors(right_index);
            }
        }
        if self.has_left_neighbor(index) {
            let left_index = index - 1;
            if self.is_open_by_index(left_index) && !self.is_full_by_index(left_index) {
                self.fullness.insert(left_index, true);
                self.fill_neighbors(left_index);
            }
        }
        if self.has_bottom_neighbor(index) {
            let diff = index & self.mask;
            let bottom_index = (((index >> self.lg_2) + 1) << self.lg_2) | diff;
            if self.is_open_by_index(bottom_index) && !self.is_full_by_index(bottom_index) {
                self.fullness.insert(bottom_index, true);
                self.fill_neighbors(bottom_index);
            }
        }
    }
}

impl Percolation for HackUnionFindPercolation {

    fn open(&mut self, row: usize, col: usize) {
        if !self.is_open(row, col) {
            let index = self.cell_index(row, col);
            self.openness.insert(index, true);
            let union = self.connect_with_top(index)
                | self.connect_with_left(index)
                | self.connect_with_right(index)
                | self.connect_with_bottom(index);
            if (union || row == 1) && self.union_find.connected(index, self.virtual_cell) {
                self.fullness.insert(index, true);
                self.fill_neighbors(index);
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
        for col in (1..).take(self.side_size) {
            if self.is_full(self.side_size, col) {
                return true;
            }
        }
        false
    }
}
