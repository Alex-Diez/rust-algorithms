#![allow(unused_mut)]

pub use algorithms::percolation::Percolation;

pub use expectest::prelude::{be_true, be_false};

describe! percolation_tests {

    before_each {
        const SIDE_SIZE: usize = 4;
        let mut percolation = Percolation::new(SIDE_SIZE);
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
        open_column_till_row(&mut percolation, 2, 1);

        expect!(percolation.is_full(2, 1)).to(be_true());
    }

    it "should not be full if a cell doesn't connect to a full cell on the top" {
        percolation.open(1, 1);
        percolation.open(2, 2);

        expect!(percolation.is_full(2, 2)).to(be_false());
    }

    it "should be full if a cell connects to a full cell on the left" {
        open_column_till_row(&mut percolation, 3, 1);
        percolation.open(3, 2);

        expect!(percolation.is_full(3, 2)).to(be_true());
    }

    it "should be full if a cell connects to a full cell on the right" {
        open_column_till_row(&mut percolation, 3, 3);
        percolation.open(3, 2);

        expect!(percolation.is_full(3, 2)).to(be_true());
    }

    it "should be full if a cell connects to a full cell on the bottom" {
        open_column_till_row(&mut percolation, SIDE_SIZE, 1);
        percolation.open(4, 2);
        percolation.open(4, 3);
        percolation.open(3, 3);

        expect!(percolation.is_full(3, 3)).to(be_true());
    }

    it "should percolate if one of bottom cells is full" {
        open_column_till_row(&mut percolation, SIDE_SIZE, 1);

        expect!(percolation.percolates()).to(be_true());
    }

    it "should not percolate when none of bottom cells is full" {
        expect!(percolation.percolates()).to(be_false());
    }

    it "should be full without matter on order of cell open operations" {
        percolation.open(SIDE_SIZE, 1);
        open_column_till_row(&mut percolation, SIDE_SIZE - 1, 1);

        expect!(percolation.is_full(SIDE_SIZE, 1)).to(be_true());
    }

    it "should not contain backwash" {
        open_column_till_row(&mut percolation, SIDE_SIZE, 1);
        percolation.open(SIDE_SIZE, SIDE_SIZE);

        expect!(percolation.is_full(SIDE_SIZE, SIDE_SIZE)).to(be_false());
    }
}

pub fn open_column_till_row(percolation: &mut Percolation, row: usize, col: usize) {
    for r in 1..row + 1 {
        percolation.open(r, col);
    }
}
