pub struct Percolation {
    openness: Vec<bool>,
    fullness: Vec<bool>,
    side_size: usize
}

impl Percolation {

    pub fn new(side_size: usize) -> Percolation {
        let length = side_size * side_size;
        let mut openness = Vec::with_capacity(length);
        let mut fullness = Vec::with_capacity(length);
        for _ in 0..length {
            openness.push(false);
            fullness.push(false);
        }
        Percolation {
            openness: openness,
            fullness: fullness,
            side_size: side_size
        }
    }

    pub fn open(&mut self, row: usize, col: usize) {
        let index = self.cell_index(row, col);
        if !self.is_open(row, col) {
            self.openness[index] = true;
            if row > 1 {
                if self.is_full(row - 1, col) {
                    self.fullness[index] = true;
                    self.fill_neighbors(row, col);
                }
            }
            if col > 1 {
                if self.is_full(row, col - 1) {
                    self.fullness[index] = true;
                    self.fill_neighbors(row, col);
                }
            }
            if col < self.side_size {
                if self.is_full(row, col + 1) {
                    self.fullness[index] = true;
                    self.fill_neighbors(row, col);
                }
            }
            if row < self.side_size {
                if self.is_full(row + 1, col) {
                    self.fullness[index] = true;
                    self.fill_neighbors(row, col);
                }
            }
            if row == 1 {
                self.fullness[index] = true;
                self.fill_neighbors(row, col);
            }
        }
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        let index = self.cell_index(row, col);
        self.openness[index]
    }

    pub fn is_full(&self, row: usize, col: usize) -> bool {
        let index = self.cell_index(row, col);
        self.is_open(row, col) && self.fullness[index]
    }

    pub fn percolates(&self) -> bool {
        for col in 1..self.side_size + 1 {
            if self.is_full(self.side_size, col) {
                return true;
            }
        }
        false
    }

    fn cell_index(&self, row: usize, col: usize) -> usize {
        (row - 1) * self.side_size + col - 1
    }

    fn fill_neighbors(&mut self, row: usize, col: usize) {
        if row > 1 && self.is_open(row - 1, col) && !self.is_full(row - 1, col) {
            let index = self.cell_index(row - 1, col);
            self.fullness[index] = true;
            self.fill_neighbors(row - 1, col);
        }
        if row < self.side_size && self.is_open(row + 1, col) && !self.is_full(row + 1, col) {
            let index = self.cell_index(row + 1, col);
            self.fullness[index] = true;
            self.fill_neighbors(row + 1, col);
        }
        if col > 1 && self.is_open(row, col - 1) && !self.is_full(row, col - 1) {
            let index = self.cell_index(row, col - 1);
            self.fullness[index] = true;
            self.fill_neighbors(row, col - 1);
        }
        if col < self.side_size && self.is_open(row, col + 1) && !self.is_full(row, col + 1) {
            let index = self.cell_index(row, col + 1);
            self.fullness[index] = true;
            self.fill_neighbors(row, col + 1);
        }
    }
}
