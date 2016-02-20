#![feature(test)]

extern crate test;
extern crate rand;
extern crate algorithms;

use rand::distributions::{IndependentSample, Range};

use algorithms::percolation::Percolation;

#[bench]
fn brute_force_percolation_small_random(bench: &mut test::Bencher) {
    let between: Range<usize> = Range::new(1, 11);
    let mut rng = rand::thread_rng();
    let mut percolation = Percolation::new(10);

    bench.iter(
        || {
            while !percolation.percolates() {
                let row = between.ind_sample(&mut rng);
                let col = between.ind_sample(&mut rng);
                percolation.open(row, col);
            }
        }
    )
}

#[bench]
fn brute_force_percolation_large_random(bench: &mut test::Bencher) {
    let between: Range<usize> = Range::new(1, 101);
    let mut rng = rand::thread_rng();
    let mut percolation = Percolation::new(100);

    bench.iter(
        || {
            while !percolation.percolates() {
                let row = between.ind_sample(&mut rng);
                let col = between.ind_sample(&mut rng);
                percolation.open(row, col);
            }
        }
    )
}
