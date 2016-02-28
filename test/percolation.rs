pub use algorithms::percolation::{Percolation, BruteForcePercolation, UnionFindPercolation, HackUnionFindPercolation};

pub use expectest::prelude::{be_true, be_false};

pub fn open_column_till_row(percolation: &mut Percolation, row: usize, col: usize) {
    for r in 1..row + 1 {
        percolation.open(r, col);
    }
}

describe! percolation_tests {

    describe! brute_force {

        before_each {
            const SIDE_SIZE: usize = 4;
            let mut percolation = BruteForcePercolation::new(SIDE_SIZE);
        }

        it "should open a cell" {
            percolation.open(1, 1);

            expect!(percolation.is_open(1, 1)).to(be_true());
        }

        it "should not be open cell which was not opened" {
            expect!(percolation.is_open(1, 1)).to(be_false());
        }

        it "should be full when opened cell is on the top" {
            percolation.open(1, 1);

            expect!(percolation.is_full(1, 1)).to(be_true());
        }

        it "should not be full cell which was not opened and locates on the top" {
            expect!(percolation.is_full(1, 1)).to(be_false());
        }

        it "should not be full when opened cell is not on the top" {
            percolation.open(2, 1);

            expect!(percolation.is_full(2, 1)).to(be_false());
        }

        it "should be full if a cell connects with cells on the top" {
            percolation.open(2, 1);
            percolation.open(1, 1);

            expect!(percolation.is_full(2, 1)).to(be_true());
        }

        it "should not be full if a cell doesn't connect to a full cell on the top" {
            percolation.open(1, 1);
            percolation.open(2, 2);

            expect!(percolation.is_full(2, 2)).to(be_false());
        }

        it "should be full if a cell connects to a full cell on the left" {
            percolation.open(3, 2);
            open_column_till_row(&mut percolation, 3, 1);

            expect!(percolation.is_full(3, 2)).to(be_true());
        }

        it "should be full if a cell connects to a full cell on the right" {
            percolation.open(3, 2);
            open_column_till_row(&mut percolation, 3, 3);

            expect!(percolation.is_full(3, 2)).to(be_true());
        }

        it "should be full if a cell connects to a full cell on the bottom" {
            percolation.open(3, 3);
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);
            percolation.open(4, 2);
            percolation.open(4, 3);

            expect!(percolation.is_full(3, 3)).to(be_true());
        }

        it "should percolate if one of bottom cells is full" {
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);

            expect!(percolation.percolates()).to(be_true());
        }

        it "should not percolate when none of bottom cells is full" {
            expect!(percolation.percolates()).to(be_false());
        }

        it "should not contain backwash" {
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);
            percolation.open(SIDE_SIZE, SIDE_SIZE);

            expect!(percolation.is_full(SIDE_SIZE, SIDE_SIZE)).to(be_false());
        }
    }

    describe! union_find {

        before_each {
            const SIDE_SIZE: usize = 10;
            let mut percolation = UnionFindPercolation::new(SIDE_SIZE);
        }

        it "should open a cell" {
            percolation.open(1, 1);

            expect!(percolation.is_open(1, 1)).to(be_true());
        }

        it "should not be open if it was not opened" {
            expect!(percolation.is_open(1, 1)).to(be_false());
        }

        it "should be full if it is a cell on the top" {
            percolation.open(1, 1);

            expect!(percolation.is_full(1, 1)).to(be_true());
        }

        it "should be full if it is an open cell not on the top" {
            percolation.open(2, 1);

            expect!(percolation.is_full(2, 1)).to(be_false());
        }

        it "should be full if it connects to a full cell on the top" {
            percolation.open(2, 1);
            percolation.open(1, 1);

            expect!(percolation.is_full(2, 1)).to(be_true());
        }

        it "should be full if it connects to a full cell on the left" {
            percolation.open(3, 4);
            open_column_till_row(&mut percolation, 3, 2);
            percolation.open(3, 3);

            expect!(percolation.is_full(3, 4)).to(be_true());
        }

        it "should be full if it connects to a full cell on the right" {
            percolation.open(4, 3);
            open_column_till_row(&mut percolation, 3, 2);
            percolation.open(4, 2);

            expect!(percolation.is_full(4, 3)).to(be_true());
        }

        it "should be full if it connects to a full cell on the bottom" {
            percolation.open(SIDE_SIZE - 1, 3);
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);
            percolation.open(SIDE_SIZE, 2);
            percolation.open(SIDE_SIZE, 3);

            expect!(percolation.is_full(SIDE_SIZE - 1, 3)).to(be_true());
        }

        it "should percolate when one of bottom cell is full" {
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);

            expect!(percolation.percolates()).to(be_true());
        }

        it "should not percolate when none of bottom cell is full" {
            expect!(percolation.percolates()).to(be_false());
        }

        it "should not contain backwash" {
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);
            percolation.open(SIDE_SIZE, SIDE_SIZE);

            expect!(percolation.is_full(SIDE_SIZE, SIDE_SIZE)).to(be_false());
        }
    }

    describe! hack_union_find {

        before_each {
            const SIDE_SIZE: usize = 10;
            let mut percolation = HackUnionFindPercolation::new(SIDE_SIZE);
        }

        it "should open a cell" {
            percolation.open(1, 1);

            expect!(percolation.is_open(1, 1)).to(be_true());
        }

        it "should not be open if it was not opened" {
            expect!(percolation.is_open(1, 1)).to(be_false());
        }

        it "should be full if it is a cell on the top" {
            percolation.open(1, 1);

            expect!(percolation.is_full(1, 1)).to(be_true());
        }

        it "should be full if it is an open cell not on the top" {
            percolation.open(2, 1);

            expect!(percolation.is_full(2, 1)).to(be_false());
        }

        it "should be full if it connects to a full cell on the top" {
            percolation.open(2, 1);
            percolation.open(1, 1);

            expect!(percolation.is_full(2, 1)).to(be_true());
        }

        it "should be full if it connects to a full cell on the left" {
            percolation.open(3, 4);
            open_column_till_row(&mut percolation, 3, 2);
            percolation.open(3, 3);

            expect!(percolation.is_full(3, 4)).to(be_true());
        }

        it "should be full if it connects to a full cell on the right" {
            percolation.open(4, 3);
            open_column_till_row(&mut percolation, 3, 2);
            percolation.open(4, 2);

            expect!(percolation.is_full(4, 3)).to(be_true());
        }

        it "should be full if it connects to a full cell on the bottom" {
            percolation.open(SIDE_SIZE - 1, 3);
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);
            percolation.open(SIDE_SIZE, 2);
            percolation.open(SIDE_SIZE, 3);

            expect!(percolation.is_full(SIDE_SIZE - 1, 3)).to(be_true());
        }

        it "should percolate when one of bottom cell is full" {
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);

            expect!(percolation.percolates()).to(be_true());
        }

        it "should not percolate when none of bottom cell is full" {
            expect!(percolation.percolates()).to(be_false());
        }

        it "should not contain backwash" {
            open_column_till_row(&mut percolation, SIDE_SIZE, 1);
            percolation.open(SIDE_SIZE, SIDE_SIZE);

            expect!(percolation.is_full(SIDE_SIZE, SIDE_SIZE)).to(be_false());
        }
    }
}
